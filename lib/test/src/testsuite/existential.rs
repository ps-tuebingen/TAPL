use super::TestSuite;
use crate::paths::{EXAMPLES_PATH, EXISTENTIAL_PATH};
use languages::Existential;
use std::path::PathBuf;

impl TestSuite for Existential {
    type Lang = Self;

    fn name(&self) -> &str {
        "Existential"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(EXISTENTIAL_PATH)
    }

    fn ext(&self) -> &str {
        "ex"
    }
}
