use super::TestSuite;
use crate::{
    paths::{EXAMPLES_PATH, F_OMEGA_PATH},
    test::TestConfig,
};
use languages::FOmega;
use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct FOmegaConf {
    ty: String,
    evaluated: String,
    name: String,
    #[serde(default)]
    contents: String,
}

impl TestConfig for FOmegaConf {
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

impl TestSuite for FOmega {
    type Config = FOmegaConf;
    type Lang = Self;

    fn name(&self) -> &str {
        "F-Omegs"
    }

    fn ext(&self) -> &str {
        "omega"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(F_OMEGA_PATH)
    }
}
