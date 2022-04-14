use handlebars::Handlebars;
use clap::Parser;

mod proto;
use proto::descriptor::*;
use proto::parser::*;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    proto: String,
    #[clap(short, long)]
    template: Option<String>
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let proto_src = std::fs::read_to_string(&args.proto)?;
    let file_descriptor = parse(&args.proto, &proto_src)?;

    match args.template {
        Some(template) => render_proto_to_template(&file_descriptor, &template),
        None => render_proto_to_json(&file_descriptor),
    }
}

fn render_proto_to_json(file_descriptor: &FileDescriptor) -> Result<(), anyhow::Error> {
    let json = serde_json::to_string_pretty(file_descriptor)?;
    println!("{}", json);
    Ok(())
}

fn render_proto_to_template(file_descriptor: &FileDescriptor, template: &str) -> Result<(), anyhow::Error> {
    let template_src = std::fs::read_to_string(template)?;
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("template", template_src)?;

    let output = handlebars.render("template", &file_descriptor)?;
    println!("{}", output);

    Ok(())
}

fn parse<'a>(file_name: &'a str, input: &'a str) -> Result<FileDescriptor<'a>, anyhow::Error> {
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

    let descriptor = FileDescriptor{
        name: file_name,
        syntax: file_syntax,
        messages: messages,
        services: services,
    };
    Ok(descriptor)
}