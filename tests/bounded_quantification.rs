use language::languages::bounded_quantification::BoundedQuantification;
use std::path::PathBuf;
use test_utils::{
    errors::Error,
    paths::{BOUNDED_PATH, EXAMPLES_PATH},
    setup,
    test::TestConfig,
    testsuite::TestSuite,
};

pub struct BoundedTests {
    source_dir: PathBuf,
}

impl BoundedTests {
    pub fn new(source_dir: PathBuf) -> BoundedTests {
        BoundedTests { source_dir }
    }
}

#[derive(serde::Deserialize)]
pub struct BoundedConf {
    ty: String,
    evaluated: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    contents: String,
}

impl TestConfig for BoundedConf {
    fn set_name(&mut self, name: String) {
        self.name = name
    }
    fn set_contents(&mut self, contents: String) {
        self.contents = contents
    }

    fn ty(&self) -> &str {
        &self.ty
    }
    fn evaluated(&self) -> &str {
        &self.evaluated
    }

    fn name(&self) -> &str {
        &self.name
    }
    fn contents(&self) -> &str {
        &self.contents
    }
}

impl TestSuite for BoundedTests {
    type Config = BoundedConf;
    type Lang = BoundedQuantification;

    fn source_dir(&self) -> PathBuf {
        self.source_dir.clone()
    }

    fn ext(&self) -> &str {
        "bd"
    }

    fn name(&self) -> &str {
        "Bounded Quantification"
    }
}

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = BoundedTests::new(examples_dir.join(BOUNDED_PATH)).run_all()?;

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
