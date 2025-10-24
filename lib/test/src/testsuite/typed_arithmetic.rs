use super::TestSuite;
use crate::paths::{EXAMPLES_PATH, TYPED_ARITH_PATH};
use languages::TypedArithmetic;
use std::path::PathBuf;

impl TestSuite for TypedArithmetic {
    type Lang = Self;

    fn name(&self) -> &str {
        "Typed Arithmetic"
    }

    fn ext(&self) -> &str {
        "arith"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(TYPED_ARITH_PATH)
    }
}
