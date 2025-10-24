use super::TestSuite;
use crate::paths::{EXAMPLES_PATH, REFERENCES_PATH};
use languages::References;
use std::path::PathBuf;

impl TestSuite for References {
    fn ext(&self) -> &str {
        "ref"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(REFERENCES_PATH)
    }
}
