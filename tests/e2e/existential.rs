use errors::test_error::TestError;
use languages::existential::Existential;
use test_utils::testsuite::TestSuite;

fn main() -> Result<(), TestError> {
    Existential.run_report()
}
