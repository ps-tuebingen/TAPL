use language::languages::untyped_arithmetic::UntypedArithmetic;
use std::path::PathBuf;
use test_utils::{
    check_test::CheckConfig,
    errors::Error,
    eval_test::EvalConfig,
    latex::LatexTestConf,
    paths::{EXAMPLES_PATH, UNTYPED_ARITH_PATH},
    setup,
    test::TestConfig,
    testsuite::TestSuite,
};
pub struct UntypedArithTests {
    source_dir: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct UntypedArithConf {
    expected: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    contents: String,
}

impl UntypedArithTests {
    pub fn new(source_dir: PathBuf) -> UntypedArithTests {
        UntypedArithTests { source_dir }
    }
}

impl TestConfig for UntypedArithConf {
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
}

impl LatexTestConf for UntypedArithConf {}
impl CheckConfig for UntypedArithConf {
    fn expected(&self) -> &str {
        ""
    }
}

impl EvalConfig for UntypedArithConf {
    fn expected(&self) -> &str {
        &self.expected
    }
}
impl TestSuite for UntypedArithTests {
    type Lang = UntypedArithmetic;
    type Config = UntypedArithConf;

    fn name(&self) -> &str {
        "Untyped Arithmetic"
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
    let fails = UntypedArithTests::new(examples_dir.join(UNTYPED_ARITH_PATH)).run_all()?;
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
