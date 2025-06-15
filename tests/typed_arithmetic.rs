use language::languages::typed_arithmetic::TypedArithmetic;
use test_utils::{errors::Error, testsuite::TestSuite};

fn main() -> Result<(), Error> {
    TypedArithmetic.run_report()
}
