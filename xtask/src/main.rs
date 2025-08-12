use errors::build_error::BuildError;

mod load_examples;

use load_examples::load_examples;

fn main() -> Result<(), BuildError> {
    load_examples()
}
