use language::languages::subtypes::Subtypes;
use test_utils::{errors::Error, testsuite::TestSuite};

fn main() -> Result<(), Error> {
    Subtypes.run_report()
}
