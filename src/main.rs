use handlebars::Handlebars;
use clap::Parser;
use handlebars::handlebars_helper;

mod proto;
use proto::descriptor::FileDescriptor;
use proto::parser::parse;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, name = "PROTO FILE", help = "Path to proto file")]
    proto: String,
    #[clap(short, long, name = "TEMPLATE FILE", help = "Path to handlebars template file")]
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
    handlebars.register_helper("snake_case", Box::new(snake_case));
    handlebars.register_helper("title_case", Box::new(title_case));
    handlebars.register_template_string("template", template_src)?;

    let output = handlebars.render("template", &file_descriptor)?;
    println!("{}", output);

    Ok(())
}

handlebars::handlebars_helper!(snake_case: |v: String| {
    let mut result: Vec<char> = vec![];
    for ch in v.chars() {
        if ch.is_uppercase() {
            result.push('_');
            result.push(ch.to_ascii_lowercase());
        } else {
            result.push(ch);
        }
    }
    result.into_iter().collect::<String>()
});

handlebars::handlebars_helper!(title_case: |v: String| {
    let mut v: Vec<char> = v.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    v.into_iter().collect::<String>()
});