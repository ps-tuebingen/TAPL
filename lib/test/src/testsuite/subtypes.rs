use super::TestSuite;
use crate::paths::{EXAMPLES_PATH, SUBTYPES_PATH};
use languages::Subtypes;
use std::path::PathBuf;

impl TestSuite for Subtypes {
    fn name(&self) -> &str {
        "Subtypes"
    }

    fn ext(&self) -> &str {
        "sub"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(SUBTYPES_PATH)
    }
}
