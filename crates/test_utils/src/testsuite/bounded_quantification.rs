use super::TestSuite;
use crate::{
    paths::{BOUNDED_PATH, EXAMPLES_PATH},
    test::TestConfig,
};
use language::languages::BoundedQuantification;
use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct BoundedConf {
    ty: String,
    evaluated: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    contents: String,
}

impl TestConfig for BoundedConf {
    fn set_name(&mut self, name: String) {
        self.name = name
    }
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

impl TestSuite for BoundedQuantification {
    type Config = BoundedConf;
    type Lang = Self;

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(BOUNDED_PATH)
    }

    fn ext(&self) -> &str {
        "bd"
    }

    fn name(&self) -> &str {
        "Bounded Quantification"
    }
}
