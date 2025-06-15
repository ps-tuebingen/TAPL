use language::languages::lambda_omega::LambdaOmega;
use std::path::PathBuf;
use test_utils::{
    errors::Error,
    paths::{EXAMPLES_PATH, LAMBDA_OMEGA_PATH},
    setup,
    test::TestConfig,
    testsuite::TestSuite,
};

pub struct LambdaOmegaTests {
    source_dir: PathBuf,
}

impl LambdaOmegaTests {
    pub fn new(source_dir: PathBuf) -> LambdaOmegaTests {
        LambdaOmegaTests { source_dir }
    }
}

#[derive(serde::Deserialize)]
pub struct LambdaOmegaConf {
    ty: String,
    evaluated: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    contents: String,
}
impl TestConfig for LambdaOmegaConf {
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

impl TestSuite for LambdaOmegaTests {
    type Config = LambdaOmegaConf;
    type Lang = LambdaOmega;

    fn name(&self) -> &str {
        "Lambda Omega"
    }

    fn ext(&self) -> &str {
        "lamo"
    }

    fn source_dir(&self) -> PathBuf {
        self.source_dir.clone()
    }
}

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = LambdaOmegaTests::new(examples_dir.join(LAMBDA_OMEGA_PATH)).run_all()?;
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
