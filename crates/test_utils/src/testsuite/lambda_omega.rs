use super::TestSuite;
use crate::{
    paths::{EXAMPLES_PATH, LAMBDA_OMEGA_PATH},
    test::TestConfig,
};
use language::languages::LambdaOmega;
use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct LambdaOmegaConf {
    ty: String,
    evaluated: String,
    name: String,
    #[serde(default)]
    contents: String,
}
impl TestConfig for LambdaOmegaConf {
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

impl TestSuite for LambdaOmega {
    type Config = LambdaOmegaConf;
    type Lang = Self;

    fn name(&self) -> &str {
        "Lambda Omega"
    }

    fn ext(&self) -> &str {
        "lamo"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(LAMBDA_OMEGA_PATH)
    }
}
