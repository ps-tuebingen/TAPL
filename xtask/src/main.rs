use errors::build_error::BuildError;

mod generate_html;
mod load_examples;

use generate_html::generate_html;
use load_examples::load_examples;

fn main() -> Result<(), BuildError> {
    load_examples()?;
    generate_html()
}
