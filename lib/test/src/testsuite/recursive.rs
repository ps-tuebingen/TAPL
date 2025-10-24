use super::TestSuite;
use crate::paths::{EXAMPLES_PATH, RECURSIVE_PATH};
use languages::Recursive;
use std::path::PathBuf;

impl TestSuite for Recursive {
    fn ext(&self) -> &str {
        "rec"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(RECURSIVE_PATH)
    }
}
