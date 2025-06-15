use language::languages::exceptions::Exceptions;
use test_utils::{errors::Error, testsuite::TestSuite};

fn main() -> Result<(), Error> {
    Exceptions.run_report()
}
