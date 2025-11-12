use errors::test_error::TestError;
use languages::bounded_quantification::BoundedQuantification;
use test_utils::testsuite::TestSuite;

fn main() -> Result<(), TestError> {
    BoundedQuantification.run_report()
}
