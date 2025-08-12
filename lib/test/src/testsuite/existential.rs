use super::TestSuite;
use crate::{
    paths::{EXAMPLES_PATH, EXISTENTIAL_PATH},
    test::TestConfig,
};
use language::Existential;
use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct ExistentialConf {
    ty: String,
    evaluated: String,
    name: String,
    #[serde(default)]
    contents: String,
}

impl TestConfig for ExistentialConf {
    fn set_contents(&mut self, contents: String) {
        self.contents = contents
    }

    fn ty(&self) -> &str {
        &self.ty
    }

    fn evaluated(&self) -> &str {
        &self.evaluated
    }

    fn name(&self) -> &str {
        &self.name
    }
    fn contents(&self) -> &str {
        &self.contents
    }
}

impl TestSuite for Existential {
    type Config = ExistentialConf;
    type Lang = Self;

    fn name(&self) -> &str {
        "Existential"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(EXISTENTIAL_PATH)
    }

    fn ext(&self) -> &str {
        "ex"
    }
}
