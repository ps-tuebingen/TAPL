use super::TestSuite;
use crate::{
    paths::{EXAMPLES_PATH, EXCEPTIONS_PATH},
    test::TestConfig,
};
use language::languages::Exceptions;
use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct ExceptionsConf {
    ty: String,
    evaluated: String,
    name: String,
    #[serde(default)]
    contents: String,
}
impl TestConfig for ExceptionsConf {
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

impl TestSuite for Exceptions {
    type Lang = Self;
    type Config = ExceptionsConf;

    fn name(&self) -> &str {
        "Exceptions"
    }

    fn ext(&self) -> &str {
        "ex"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(EXCEPTIONS_PATH)
    }
}
