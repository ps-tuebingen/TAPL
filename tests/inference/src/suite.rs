use std::path::PathBuf;
use test_common::{
    errors::Error,
    load_tests::{load_dir, TestContents},
    testsuite::{Test, TestSuite},
};

pub struct InferenceTests {
    source_path: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct InferenceConf {}

impl InferenceTests {
    pub fn new(path: PathBuf) -> InferenceTests {
        InferenceTests { source_path: path }
    }
}

impl TestSuite for InferenceTests {
    fn name(&self) -> String {
        "Type Inference".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<InferenceConf>> = load_dir(&self.source_path, "inf")?;
        let mut tests = vec![];
        for tst in contents {}
        Ok(tests)
    }
}
