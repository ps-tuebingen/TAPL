use language::languages::stlc::Stlc;
use std::path::PathBuf;
use test_utils::{
    errors::Error,
    paths::{EXAMPLES_PATH, STLC_PATH},
    setup,
    test::TestConfig,
    testsuite::TestSuite,
};

pub struct StlcTests {
    source_dir: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct StlcConf {
    ty: String,
    evaled: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    contents: String,
}

impl StlcTests {
    pub fn new(source_dir: PathBuf) -> StlcTests {
        StlcTests { source_dir }
    }
}

impl TestConfig for StlcConf {
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
        &self.evaled
    }
}

impl TestSuite for StlcTests {
    type Config = StlcConf;
    type Lang = Stlc;

    fn name(&self) -> &str {
        "Stlc"
    }

    fn ext(&self) -> &str {
        "stlc"
    }

    fn source_dir(&self) -> PathBuf {
        self.source_dir.clone()
    }
}
fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = StlcTests::new(examples_dir.join(STLC_PATH)).run_all()?;
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
