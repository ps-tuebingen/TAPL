use super::TestSuite;
use crate::{
    paths::{EXAMPLES_PATH, STLC_PATH},
    test::TestConfig,
};
use language::languages::Stlc;
use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct StlcConf {
    ty: String,
    evaled: String,
    name: String,
    #[serde(default)]
    contents: String,
}

impl TestConfig for StlcConf {
    fn set_contents(&mut self, contents: String) {
        self.contents = contents
    }

    fn name(&self) -> &str {
        &self.name
    }
    fn contents(&self) -> &str {
        &self.contents
    }
    fn ty(&self) -> &str {
        &self.ty
    }
    fn evaluated(&self) -> &str {
        &self.evaled
    }
}

impl TestSuite for Stlc {
    type Config = StlcConf;
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
