use language::languages::existential::Existential;
use test_utils::{errors::Error, testsuite::TestSuite};

fn main() -> Result<(), Error> {
    Existential.run_report()
}
