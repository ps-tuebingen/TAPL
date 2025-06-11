use language::languages::f_omega::FOmega;
use std::path::PathBuf;
use test_utils::{
    check_test::CheckConfig,
    errors::Error,
    eval_test::EvalConfig,
    latex::LatexTestConf,
    paths::{EXAMPLES_PATH, F_OMEGA_PATH},
    setup,
    test::TestConfig,
    testsuite::TestSuite,
};

pub struct FOmegaTests {
    source_dir: PathBuf,
}

impl FOmegaTests {
    pub fn new(source_dir: PathBuf) -> FOmegaTests {
        FOmegaTests { source_dir }
    }
}

#[derive(serde::Deserialize)]
pub struct FOmegaConf {
    ty: String,
    evaluated: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    contents: String,
}

impl TestConfig for FOmegaConf {
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

impl LatexTestConf for FOmegaConf {}

impl CheckConfig for FOmegaConf {
    fn expected(&self) -> &str {
        &self.ty
    }
}

impl EvalConfig for FOmegaConf {
    fn expected(&self) -> &str {
        &self.evaluated
    }
}

impl TestSuite for FOmegaTests {
    type Config = FOmegaConf;
    type Lang = FOmega;

    fn name(&self) -> &str {
        "F-Omegs"
    }

    fn ext(&self) -> &str {
        "omega"
    }

    fn source_dir(&self) -> PathBuf {
        self.source_dir.clone()
    }
}

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = FOmegaTests::new(examples_dir.join(F_OMEGA_PATH)).run_all()?;
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
