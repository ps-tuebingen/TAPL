use super::TestSuite;
use crate::{
    paths::{EXAMPLES_PATH, SUBTYPES_PATH},
    test::TestConfig,
};
use language::languages::Subtypes;
use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct SubtypesConf {
    ty: String,
    evaluated: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    contents: String,
}

impl TestConfig for SubtypesConf {
    fn set_name(&mut self, name: String) {
        self.name = name
    }
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
impl TestSuite for Subtypes {
    type Config = SubtypesConf;
    type Lang = Self;

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
