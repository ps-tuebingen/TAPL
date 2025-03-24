use std::path::PathBuf;
use test_common::{
    errors::Error,
    load_tests::{load_dir, TestContents},
    testsuite::{Test, TestSuite},
};

pub struct FeatherweightTests {
    source_path: PathBuf,
}

#[derive(serde::Deserialize)]
struct FeatherweightConf {
    ty: String,
    evaluated: String,
}

impl FeatherweightTests {
    pub fn new(path: PathBuf) -> FeatherweightTests {
        FeatherweightTests { source_path: path }
    }
}

impl TestSuite for FeatherweightTests {
    fn name(&self) -> String {
        "Featherweight Java".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<FeatherweightConf>> = load_dir(&self.source_path, "java")?;
        let mut tests = vec![];
        for tst in contents {}
        Ok(tests)
    }
}
