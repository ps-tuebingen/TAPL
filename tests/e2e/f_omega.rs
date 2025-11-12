use errors::test_error::TestError;
use languages::f_omega::FOmega;
use test_utils::testsuite::TestSuite;

fn main() -> Result<(), TestError> {
    FOmega.run_report()
}
