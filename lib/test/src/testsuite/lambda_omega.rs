use super::TestSuite;
use crate::paths::{EXAMPLES_PATH, LAMBDA_OMEGA_PATH};
use languages::LambdaOmega;
use std::path::PathBuf;

impl TestSuite for LambdaOmega {
    fn ext(&self) -> &str {
        "lamo"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(LAMBDA_OMEGA_PATH)
    }
}
