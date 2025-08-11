use errors::test_error::TestError;
use language::languages::recursive::Recursive;
use test_utils::testsuite::TestSuite;

fn main() -> Result<(), TestError> {
    Recursive.run_report()
}
