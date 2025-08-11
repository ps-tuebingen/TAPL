use super::TestSuite;
use crate::{
    paths::{EXAMPLES_PATH, UNTYPED_ARITH_PATH},
    test::TestConfig,
};
use language::UntypedArithmetic;
use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct UntypedArithConf {
    expected: String,
    name: String,
    #[serde(default)]
    contents: String,
}

impl TestConfig for UntypedArithConf {
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
        &self.expected
    }
}

impl TestSuite for UntypedArithmetic {
    type Lang = Self;
    type Config = UntypedArithConf;

    fn name(&self) -> &str {
        "Untyped Arithmetic"
    }

    fn ext(&self) -> &str {
        "arith"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(UNTYPED_ARITH_PATH)
    }
}
