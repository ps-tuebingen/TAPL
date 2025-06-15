use language::languages::untyped_arithmetic::UntypedArithmetic;
use test_utils::{errors::Error, testsuite::TestSuite};

fn main() -> Result<(), Error> {
    UntypedArithmetic.run_report()
}
