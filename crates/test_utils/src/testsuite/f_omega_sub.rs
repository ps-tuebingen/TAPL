use super::TestSuite;
use crate::{
    paths::{EXAMPLES_PATH, F_OMEGA_SUB_PATH},
    test::TestConfig,
};
use language::languages::FOmegaSub;
use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct FOmegaSubConf {
    ty: String,
    evaluated: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    contents: String,
}

impl TestConfig for FOmegaSubConf {
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

impl TestSuite for FOmegaSub {
    type Lang = Self;
    type Config = FOmegaSubConf;

    fn name(&self) -> &str {
        "Existential"
    }

    fn ext(&self) -> &str {
        "omega"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(F_OMEGA_SUB_PATH)
    }
}
