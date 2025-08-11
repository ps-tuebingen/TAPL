use errors::test_error::TestError;
use language::languages::untyped_arithmetic::UntypedArithmetic;
use test_utils::testsuite::TestSuite;

fn main() -> Result<(), TestError> {
    UntypedArithmetic.run_report()
}
