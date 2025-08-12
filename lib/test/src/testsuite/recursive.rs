use super::TestSuite;
use crate::{
    paths::{EXAMPLES_PATH, RECURSIVE_PATH},
    test::TestConfig,
};
use languages::Recursive;
use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct RecursiveConf {
    ty: String,
    evaluated: String,
    name: String,
    #[serde(default)]
    contents: String,
}

impl TestConfig for RecursiveConf {
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

impl TestSuite for Recursive {
    type Config = RecursiveConf;
    type Lang = Self;

    fn name(&self) -> &str {
        "Recursive"
    }

    fn ext(&self) -> &str {
        "rec"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(RECURSIVE_PATH)
    }
}
