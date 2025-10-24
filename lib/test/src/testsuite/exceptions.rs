use super::TestSuite;
use crate::paths::{EXAMPLES_PATH, EXCEPTIONS_PATH};
use languages::Exceptions;
use std::path::PathBuf;

impl TestSuite for Exceptions {
    fn name(&self) -> &str {
        "Exceptions"
    }

    fn ext(&self) -> &str {
        "ex"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(EXCEPTIONS_PATH)
    }
}
