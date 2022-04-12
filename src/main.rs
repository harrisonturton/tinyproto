use std::fs::File;
use handlebars::{Handlebars, to_json};
use clap::Parser;
use anyhow::Error;
use serde_json::{Map, Value as Json};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    input: String,

    #[clap(short, long)]
    output: String,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    println!("Reading from {}", args.input);

    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("template", args.input)?;

    let data = make_data();

    let mut output_file = File::create(args.output)?;
    handlebars.render_to_write("template", &data, &mut output_file)?;
    println!("Generated!");

    Ok(())
}

fn make_data() -> Map<String, Json> {
    let mut data = Map::new();
    data.insert("function_name".to_owned(), to_json("say_hello"));
    data
}