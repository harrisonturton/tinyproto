use handlebars::Handlebars;
use std::fs::File;
use serde_json::{Map, Value};
use anyhow::Error;

pub fn write_file_from_template(
    template_path: String,
    output_path: String,
    data: Map<String, Value>,
) -> Result<(), Error> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("template", template_path)?;

    let mut output_file = File::create(output_path)?;
    handlebars.render_to_write("template", &data, &mut output_file)?;
    Ok(())
}