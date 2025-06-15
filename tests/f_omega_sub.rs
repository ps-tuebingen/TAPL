use language::languages::f_omega_sub::FOmegaSub;
use test_utils::{errors::Error, testsuite::TestSuite};

fn main() -> Result<(), Error> {
    FOmegaSub.run_report()
}
