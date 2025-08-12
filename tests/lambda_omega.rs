use errors::test_error::TestError;
use languages::lambda_omega::LambdaOmega;
use test_utils::testsuite::TestSuite;

fn main() -> Result<(), TestError> {
    LambdaOmega.run_report()
}
