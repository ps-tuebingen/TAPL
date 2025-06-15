use language::languages::system_f::SystemF;
use std::path::PathBuf;
use test_utils::{
    errors::Error,
    paths::{EXAMPLES_PATH, SYSTEMF_PATH},
    setup,
    test::TestConfig,
    testsuite::TestSuite,
};

pub struct SystemFTests {
    source_path: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct SystemFConf {
    ty: String,
    evaluated: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    contents: String,
}

impl SystemFTests {
    pub fn new(path: PathBuf) -> SystemFTests {
        SystemFTests { source_path: path }
    }
}

impl TestConfig for SystemFConf {
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

impl TestSuite for SystemFTests {
    type Lang = SystemF;
    type Config = SystemFConf;

    fn name(&self) -> &str {
        "System F"
    }

    fn ext(&self) -> &str {
        "f"
    }

    fn source_dir(&self) -> PathBuf {
        self.source_path.clone()
    }
}

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = SystemFTests::new(examples_dir.join(SYSTEMF_PATH)).run_all()?;
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
