use language::languages::system_f::SystemF;
use test_utils::{errors::Error, testsuite::TestSuite};

fn main() -> Result<(), Error> {
    SystemF.run_report()
}
