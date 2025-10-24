use super::TestSuite;
use crate::paths::{EXAMPLES_PATH, F_OMEGA_PATH};
use languages::FOmega;
use std::path::PathBuf;

impl TestSuite for FOmega {
    fn ext(&self) -> &str {
        "omega"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(F_OMEGA_PATH)
    }
}
