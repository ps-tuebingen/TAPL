use errors::test_error::TestError;
use language::languages::references::References;
use test_utils::testsuite::TestSuite;

fn main() -> Result<(), TestError> {
    References.run_report()
}
