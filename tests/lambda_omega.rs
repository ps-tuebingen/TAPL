use language::languages::lambda_omega::LambdaOmega;
use test_utils::{errors::Error, testsuite::TestSuite};

fn main() -> Result<(), Error> {
    LambdaOmega.run_report()
}
