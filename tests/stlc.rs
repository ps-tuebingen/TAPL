use errors::test_error::TestError;
use language::languages::stlc::Stlc;
use test_utils::testsuite::TestSuite;

fn main() -> Result<(), TestError> {
    Stlc.run_report()
}
