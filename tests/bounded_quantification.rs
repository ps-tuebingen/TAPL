use language::languages::bounded_quantification::BoundedQuantification;
use test_utils::{errors::Error, testsuite::TestSuite};

fn main() -> Result<(), Error> {
    BoundedQuantification.run_report()
}
