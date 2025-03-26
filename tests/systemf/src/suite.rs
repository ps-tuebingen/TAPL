use std::path::PathBuf;
use test_common::{
    errors::Error,
    load_tests::{load_dir, TestContents},
    testsuite::{Test, TestSuite},
};

pub struct SystemFTests {
    source_path: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct SystemFConf {}

impl SystemFTests {
    pub fn new(path: PathBuf) -> SystemFTests {
        SystemFTests { source_path: path }
    }
}

impl TestSuite for SystemFTests {
    fn name(&self) -> String {
        "System F".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<SystemFConf>> = load_dir(&self.source_path, "f")?;
        let tests = vec![];
        Ok(tests)
    }
}
