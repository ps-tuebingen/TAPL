use errors::test_error::TestError;
use language::languages::subtypes::Subtypes;
use test_utils::testsuite::TestSuite;

fn main() -> Result<(), TestError> {
    Subtypes.run_report()
}
