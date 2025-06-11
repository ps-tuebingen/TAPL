use language::languages::untyped_lambda::UntypedLambda;
use std::path::PathBuf;
use test_utils::{
    check_test::CheckConfig,
    errors::Error,
    eval_test::EvalConfig,
    latex::LatexTestConf,
    paths::{EXAMPLES_PATH, UNTYPED_LAMBDA_PATH},
    setup,
    test::TestConfig,
    testsuite::TestSuite,
};
pub struct UntypedLambdaTests {
    source_dir: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct UntypedLambdaConf {
    evaluated: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    contents: String,
}

impl TestConfig for UntypedLambdaConf {
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

impl LatexTestConf for UntypedLambdaConf {}
impl CheckConfig for UntypedLambdaConf {
    fn expected(&self) -> &str {
        ""
    }
}

impl EvalConfig for UntypedLambdaConf {
    fn expected(&self) -> &str {
        &self.evaluated
    }
}

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = UntypedLambdaTests::new(examples_dir.join(UNTYPED_LAMBDA_PATH)).run_all()?;
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

impl TestSuite for UntypedLambdaTests {
    type Lang = UntypedLambda;
    type Config = UntypedLambdaConf;

    fn name(&self) -> &str {
        "Untyped Lambda"
    }

    fn ext(&self) -> &str {
        "lam"
    }

    fn source_dir(&self) -> PathBuf {
        self.source_dir.clone()
    }
}

impl UntypedLambdaTests {
    pub fn new(source_dir: PathBuf) -> UntypedLambdaTests {
        UntypedLambdaTests { source_dir }
    }
}
