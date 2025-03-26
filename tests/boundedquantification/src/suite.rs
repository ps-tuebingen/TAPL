use std::path::PathBuf;
use test_common::{
    errors::Error,
    load_tests::{load_dir, TestContents},
    testsuite::{Test, TestSuite},
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
pub struct NamelessConfig {}

impl TestSuite for BoundedTests {
    fn name(&self) -> String {
        "Nameless Representation".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<NamelessConfig>> = load_dir(&self.source_dir, "lam")?;
        let mut tests = vec![];
        for source in contents {}
        Ok(tests)
    }
}
