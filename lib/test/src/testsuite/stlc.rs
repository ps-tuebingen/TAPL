use super::TestSuite;
use crate::paths::{EXAMPLES_PATH, STLC_PATH};
use languages::Stlc;
use std::path::PathBuf;

impl TestSuite for Stlc {
    type Lang = Self;

    fn name(&self) -> &str {
        "Stlc"
    }

    fn ext(&self) -> &str {
        "stlc"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(STLC_PATH)
    }
}
