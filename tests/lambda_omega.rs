use language::languages::lambda_omega::LambdaOmega;
use test_utils::{errors::Error, setup, testsuite::TestSuite};

fn main() -> Result<(), Error> {
    setup()?;

    let fails = LambdaOmega.run_all()?;
    let fail_str = if fails > 0 {
        format!("\x1b[31m{fails} fails\x1b[39m")
    } else {
        "0 fails".to_owned()
    };
    println!("Finished running tests with {fail_str}");
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}
