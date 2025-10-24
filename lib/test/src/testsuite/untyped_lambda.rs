use super::TestSuite;
use crate::paths::{EXAMPLES_PATH, UNTYPED_LAMBDA_PATH};
use languages::UntypedLambda;
use std::path::PathBuf;

impl TestSuite for UntypedLambda {
    type Lang = Self;

    fn name(&self) -> &str {
        "Untyped Lambda"
    }

    fn ext(&self) -> &str {
        "lam"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(UNTYPED_LAMBDA_PATH)
    }
}
