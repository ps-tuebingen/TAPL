use language::languages::untyped_lambda::UntypedLambda;
use test_utils::{errors::Error, testsuite::TestSuite};

fn main() -> Result<(), Error> {
    UntypedLambda.run_report()
}
