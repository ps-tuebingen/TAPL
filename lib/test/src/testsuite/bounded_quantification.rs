use super::TestSuite;
use crate::paths::{BOUNDED_PATH, EXAMPLES_PATH};
use languages::BoundedQuantification;
use std::path::PathBuf;

impl TestSuite for BoundedQuantification {
    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(BOUNDED_PATH)
    }

    fn ext(&self) -> &str {
        "bd"
    }
}
