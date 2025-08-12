use super::TestSuite;
use crate::{
    paths::{EXAMPLES_PATH, SYSTEMF_PATH},
    test::TestConfig,
};
use languages::SystemF;
use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct SystemFConf {
    ty: String,
    evaluated: String,
    name: String,
    #[serde(default)]
    contents: String,
}

impl TestConfig for SystemFConf {
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
        &self.evaluated
    }
}

impl TestSuite for SystemF {
    type Lang = SystemF;
    type Config = SystemFConf;

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
