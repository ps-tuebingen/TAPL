use errors::test_error::TestError;
use languages::typed_arithmetic::TypedArithmetic;
use test_utils::testsuite::TestSuite;

fn main() -> Result<(), TestError> {
    TypedArithmetic.run_report()
}
