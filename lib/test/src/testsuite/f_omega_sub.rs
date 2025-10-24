use super::TestSuite;
use crate::paths::{EXAMPLES_PATH, F_OMEGA_SUB_PATH};
use languages::FOmegaSub;
use std::path::PathBuf;

impl TestSuite for FOmegaSub {
    fn ext(&self) -> &str {
        "omega"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(F_OMEGA_SUB_PATH)
    }
}
