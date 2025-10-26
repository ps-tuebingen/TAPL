use errors::test_error::TestError;
use languages::untyped_lambda::UntypedLambda;
use test_utils::testsuite::TestSuite;

fn main() -> Result<(), TestError> {
    UntypedLambda.run_report()
}
