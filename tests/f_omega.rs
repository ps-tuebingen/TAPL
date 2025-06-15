use language::languages::f_omega::FOmega;
use test_utils::{errors::Error, testsuite::TestSuite};

fn main() -> Result<(), Error> {
    FOmega.run_report()
}
