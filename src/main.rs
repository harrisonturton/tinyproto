use nom::multi::many0;

mod proto;
use proto::descriptor::*;
use proto::parser::*;

fn main() -> Result<(), anyhow::Error> {
    let input = r#"
        syntax = "proto3";

        message MyRequest {
            required string name = 1;
            string age = 2;
        }

        message MyResponse {
            required string name = 1;
            string age = 2;
        }

        syntax = "proto2";

        message Testing {
            required string name = 1;
            string age = 2;
        }

        service MyService {
            rpc MyRpc(SomeType) returns (stream OtherType);
        }
    "#;

    let tokens = proto::token::tokenize(input)?;
    println!("{:?}", tokens);

    Ok(())
}

fn main2() -> Result<(), anyhow::Error> {
    let input = r#"
    
        syntax = "proto3";

        message MyRequest {
            required string name = 1;
            string age = 2;
        }

        message MyResponse {
            required string name = 1;
            string age = 2;
        }

        syntax = "proto2";

        message Testing {
            required string name = 1;
            string age = 2;
        }

        service MyService {
            rpc MyRpc(SomeType) returns (stream OtherType);
        }
    "#;

    let mut syntax: Option<&str> = None;
    let mut messages: Vec<MessageDescriptor> = vec![];
    let mut services: Vec<ServiceDescriptor> = vec![];

    let mut remaining = input;
    while !remaining.is_empty() {
        // If we encounter a syntax version declaration
        if let Ok(result) = syntax_statement(remaining) {
            let (rest, version) = result;
            syntax = Some(version);
            remaining = rest;
            continue;
        }

        // If we encounter a message declaration
        if let Ok(result) = message_header(remaining) {
            let (rest, msg_name) = result;
            let (rest, msg_body) = brace_delimited(rest)?;

            let (_, msg_fields) = many0(message_field)(msg_body)?;
            let mut fields = vec![];
            for (label, typ, name, number) in msg_fields {
                let field = FieldDescriptor {
                    label: parse_field_descriptor_label(&label),
                    typ: parse_field_descriptor_typ(typ),
                    name: name.to_string(),
                    number: number.to_string().parse::<u32>().unwrap(),
                };
                fields.push(field);
            }

            let message = MessageDescriptor {
                name: msg_name.to_owned(),
                fields: fields,
            };
            messages.push(message);

            remaining = rest;
            continue;
        }

        // If we encounter a service declaration
        if let Ok(result) = service_header(remaining) {
            let (rest, service_name) = result;
            let (rest, service_body) = brace_delimited(rest)?;

            let (_, raw_methods) = many0(service_method)(service_body)?;
            let mut methods = vec![];
            for (name, (client_stream, input_type), (server_stream, output_type)) in raw_methods {
                let method = MethodDescriptor {
                    name: name.to_string(),
                    input_type: input_type.to_string(),
                    output_type: output_type.to_string(),
                    client_streaming: client_stream.is_some(),
                    server_streaming: server_stream.is_some(),
                };
                methods.push(method);
            }
            
            let service = ServiceDescriptor {
                name: service_name.to_string(),
                methods: methods,
            };
            services.push(service);

            remaining = rest;
            continue;
        }

        println!("No match!");
        return Err(anyhow::anyhow!("Failed to match anything"));
    }

    let syntax = match syntax {
        Some("proto2") => Some(SyntaxDescriptor::Proto2),
        Some("proto3") => Some(SyntaxDescriptor::Proto3),
        None => None,
        _ => return Err(anyhow::anyhow!("unknown syntax version")),
    };

    let file = FileDescriptor{
        name: String::from("File name"),
        syntax: syntax,
        messages: messages,
        services: services,
    };
    println!("{:?}", file);

    Ok(())
}

fn parse_field_descriptor_label(label: &Option<&str>) -> Option<FieldDescriptorLabel> {
    let val = match label {
        Some(val) => *val,
        None => return None,
    };
    match val {
        "optional" => Some(FieldDescriptorLabel::Optional),
        "required" => Some(FieldDescriptorLabel::Required),
        "repeated" => Some(FieldDescriptorLabel::Repeated),
        unknown => Some(FieldDescriptorLabel::Unknown(unknown.to_owned())),
    }
}

fn parse_field_descriptor_typ(typ: &str) -> FieldDescriptorType {
    match typ {
        "string" => FieldDescriptorType::String,
        message => FieldDescriptorType::Message(message.to_owned()),
    }
}