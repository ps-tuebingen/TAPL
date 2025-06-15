use language::languages::typed_arithmetic::TypedArithmetic;
use std::path::PathBuf;
use test_utils::{
    errors::Error,
    paths::{EXAMPLES_PATH, TYPED_ARITH_PATH},
    setup,
    test::TestConfig,
    testsuite::TestSuite,
};

pub struct TypedArithTests {
    source_dir: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct TypedArithConf {
    ty: String,
    expected: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    contents: String,
}

impl TestConfig for TypedArithConf {
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
        &self.expected
    }
}

impl TypedArithTests {
    pub fn new(source_dir: PathBuf) -> TypedArithTests {
        TypedArithTests { source_dir }
    }
}
impl TestSuite for TypedArithTests {
    type Config = TypedArithConf;
    type Lang = TypedArithmetic;

    fn name(&self) -> &str {
        "Typed Arithmetic"
    }

    fn ext(&self) -> &str {
        "arith"
    }

    fn source_dir(&self) -> PathBuf {
        self.source_dir.clone()
    }
}

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = TypedArithTests::new(examples_dir.join(TYPED_ARITH_PATH)).run_all()?;
    let fail_str = if fails > 0 {
        format!("\x1b[31m{fails} fails\x1b[39m")
    } else {
        "0 fails".to_owned()
    };
    println!("Finished running tests with {fail_str}");
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}
