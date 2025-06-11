use language::languages::existential::Existential;
use std::path::PathBuf;
use test_utils::{
    check_test::CheckConfig,
    errors::Error,
    eval_test::EvalConfig,
    latex::LatexTestConf,
    paths::{EXAMPLES_PATH, EXISTENTIAL_PATH},
    setup,
    test::TestConfig,
    testsuite::TestSuite,
};

pub struct ExistentialTests {
    source_dir: PathBuf,
}

impl ExistentialTests {
    pub fn new(source_dir: PathBuf) -> ExistentialTests {
        ExistentialTests { source_dir }
    }
}

#[derive(serde::Deserialize)]
pub struct ExistentialConf {
    ty: String,
    evaluated: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    contents: String,
}

impl TestConfig for ExistentialConf {
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
impl LatexTestConf for ExistentialConf {}
impl CheckConfig for ExistentialConf {
    fn expected(&self) -> &str {
        &self.ty
    }
}
impl EvalConfig for ExistentialConf {
    fn expected(&self) -> &str {
        &self.evaluated
    }
}

impl TestSuite for ExistentialTests {
    type Config = ExistentialConf;
    type Lang = Existential;

    fn name(&self) -> &str {
        "Existential"
    }

    fn source_dir(&self) -> PathBuf {
        self.source_dir.clone()
    }

    fn ext(&self) -> &str {
        "ex"
    }
}

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = ExistentialTests::new(examples_dir.join(EXISTENTIAL_PATH)).run_all()?;

    let fail_str = if fails > 0 {
        format!("\x1b[31m{fails} fails\x1b[39m")
    } else {
        "0 fails".to_owned()
    };
    println!("Finished running tests with {fail_str}",);
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}
