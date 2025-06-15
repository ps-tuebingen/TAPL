use language::languages::recursive::Recursive;
use test_utils::{errors::Error, testsuite::TestSuite};

fn main() -> Result<(), Error> {
    Recursive.run_report()
}
