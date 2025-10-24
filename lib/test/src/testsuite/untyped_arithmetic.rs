use super::TestSuite;
use crate::paths::{EXAMPLES_PATH, UNTYPED_ARITH_PATH};
use languages::UntypedArithmetic;
use std::path::PathBuf;

impl TestSuite for UntypedArithmetic {
    fn ext(&self) -> &str {
        "arith"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(UNTYPED_ARITH_PATH)
    }
}
