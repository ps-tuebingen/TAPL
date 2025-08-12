use super::TestSuite;
use crate::{
    paths::{EXAMPLES_PATH, TYPED_ARITH_PATH},
    test::TestConfig,
};
use language::TypedArithmetic;
use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct TypedArithConf {
    ty: String,
    expected: String,
    name: String,
    #[serde(default)]
    contents: String,
}

impl TestConfig for TypedArithConf {
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
        &self.expected
    }
}

impl TestSuite for TypedArithmetic {
    type Config = TypedArithConf;
    type Lang = Self;

    fn name(&self) -> &str {
        "Typed Arithmetic"
    }

    fn ext(&self) -> &str {
        "arith"
    }

    fn source_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_PATH).join(TYPED_ARITH_PATH)
    }
}
