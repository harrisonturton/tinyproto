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

    let mut file_syntax: Option<SyntaxDescriptor> = None;
    let mut messages: Vec<MessageDescriptor> = vec![];
    let mut services: Vec<ServiceDescriptor> = vec![];

    let mut remaining = input;
    while !remaining.is_empty() {
        // If we encounter a syntax version declaration
        if let Ok(result) = syntax(remaining) {
            let (rest, syntax) = result;
            file_syntax = Some(syntax);
            remaining = rest;
            continue;
        }

        // If we encounter a message declaration
        if let Ok(result) = message(remaining) {
            let (rest, message) = result;
            messages.push(message);
            remaining = rest;
            continue;
        }

        // If we encounter a service declaration
        if let Ok(result) = service(remaining) {
            let (rest, service) = result;
            services.push(service);
            remaining = rest;
            continue;
        }

        println!("No match!");
        return Err(anyhow::anyhow!("Unexpected characters {}", remaining));
    }

    let file = FileDescriptor{
        name: "File name",
        syntax: file_syntax,
        messages: messages,
        services: services,
    };

    let json = serde_json::to_string(&file)?;
    println!("{}", json);

    Ok(())
}