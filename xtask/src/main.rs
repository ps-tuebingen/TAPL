use errors::build_error::BuildError;

mod generate_html;
mod load_examples;
mod web_config;

use generate_html::generate_html;
use load_examples::load_examples;
use web_config::load_web_config;

fn main() -> Result<(), BuildError> {
    load_examples()?;
    load_web_config()?;
    generate_html()?;
    Ok(())
}
