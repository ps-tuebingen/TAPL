use language::languages::stlc::Stlc;
use test_utils::{errors::Error, testsuite::TestSuite};

fn main() -> Result<(), Error> {
    Stlc.run_report()
}
