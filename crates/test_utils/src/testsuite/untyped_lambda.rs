use super::TestSuite;
use crate::{
    paths::{EXAMPLES_PATH, UNTYPED_LAMBDA_PATH},
    test::TestConfig,
};
use language::languages::UntypedLambda;
use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct UntypedLambdaConf {
    evaluated: String,
    name: String,
    #[serde(default)]
    contents: String,
}
impl TestConfig for UntypedLambdaConf {
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
        ""
    }
    fn evaluated(&self) -> &str {
        &self.evaluated
    }
}

impl TestSuite for UntypedLambda {
    type Lang = Self;
    type Config = UntypedLambdaConf;

    fn name(&self) -> &str {
        "Untyped Lambda"
    }

    fn ext(&self) -> &str {
        "lam"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(UNTYPED_LAMBDA_PATH)
    }
}
