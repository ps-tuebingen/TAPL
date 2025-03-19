use std::{
    env::{current_dir, set_current_dir},
    path::PathBuf,
};

mod errors;
mod paths;
mod test;
mod testsuite;

mod nameless;
mod stlc;
mod untyped_arithmetic;
mod untyped_lambda;

use errors::Error;
use nameless::NamelessRepTests;
use paths::{EXAMPLES_PATH, STLC_PATH, UNTYPED_ARITH_PATH, UNTYPED_LAMBDA_PATH};
use stlc::StlcTests;
use testsuite::TestSuite;
use untyped_arithmetic::UntypedArithTests;
use untyped_lambda::UntypedLambdaTests;

fn setup() -> Result<(), Error> {
    let dir = current_dir()
        .map_err(|_| Error::GetCurrentDir)?
        .join("..")
        .join("..");
    set_current_dir(dir).map_err(|_| Error::SetCurrentDir)?;
    Ok(())
}

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let mut fails = 0;

    fails += UntypedArithTests::new(examples_dir.join(UNTYPED_ARITH_PATH)).run_all()?;
    fails += UntypedLambdaTests::new(examples_dir.join(UNTYPED_LAMBDA_PATH)).run_all()?;
    fails += NamelessRepTests::new(examples_dir.join(UNTYPED_LAMBDA_PATH)).run_all()?;
    fails += StlcTests::new(examples_dir.join(STLC_PATH)).run_all()?;

    println!(
        "Finished running tests with \x1b[31m{} fails\x1b[39m",
        fails
    );
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}
