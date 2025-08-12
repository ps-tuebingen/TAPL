use errors::test_error::TestError;
use languages::f_omega_sub::FOmegaSub;
use test_utils::testsuite::TestSuite;

fn main() -> Result<(), TestError> {
    FOmegaSub.run_report()
}
