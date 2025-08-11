use errors::test_error::TestError;
use language::languages::exceptions::Exceptions;
use test_utils::testsuite::TestSuite;

fn main() -> Result<(), TestError> {
    Exceptions.run_report()
}
