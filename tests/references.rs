use language::languages::references::References;
use test_utils::{errors::Error, testsuite::TestSuite};

fn main() -> Result<(), Error> {
    References.run_report()
}
