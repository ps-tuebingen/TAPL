use super::TestSuite;
use crate::paths::{EXAMPLES_PATH, SYSTEMF_PATH};
use languages::SystemF;
use std::path::PathBuf;

impl TestSuite for SystemF {
    type Lang = SystemF;

    fn name(&self) -> &str {
        "System F"
    }

    fn ext(&self) -> &str {
        "f"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(SYSTEMF_PATH)
    }
}
