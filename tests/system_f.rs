use errors::test_error::TestError;
use language::languages::system_f::SystemF;
use test_utils::testsuite::TestSuite;

fn main() -> Result<(), TestError> {
    SystemF.run_report()
}
