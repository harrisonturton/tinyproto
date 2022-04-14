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
    template: String
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let input = std::fs::read_to_string(args.proto)?;
    let file_descriptor = parse(&input)?;

    let json = serde_json::to_string_pretty(&file_descriptor)?;
    println!("{}", json);

    Ok(())
}

fn parse(input: &str) -> Result<FileDescriptor, anyhow::Error> {
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
        name: "File name",
        syntax: file_syntax,
        messages: messages,
        services: services,
    };
    Ok(descriptor)
}