use super::TestSuite;
use crate::{
    paths::{EXAMPLES_PATH, REFERENCES_PATH},
    test::TestConfig,
};
use language::languages::References;
use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct ReferencesConf {
    ty: String,
    evaluated: String,
    name: String,
    #[serde(default)]
    contents: String,
}

impl TestConfig for ReferencesConf {
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

impl TestSuite for References {
    type Config = ReferencesConf;
    type Lang = Self;

    fn name(&self) -> &str {
        "References"
    }

    fn ext(&self) -> &str {
        "ref"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(REFERENCES_PATH)
    }
}
