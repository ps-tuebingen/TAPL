use std::path::PathBuf;
use test_common::{
    errors::Error,
    load_tests::{load_dir, TestContents},
    testsuite::{Test, TestSuite},
};

pub struct SubtypesTests {
    source_path: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct SubtypesConf {
    ty: String,
}

impl SubtypesTests {
    pub fn new(path: PathBuf) -> SubtypesTests {
        SubtypesTests { source_path: path }
    }
}

impl TestSuite for SubtypesTests {
    fn name(&self) -> String {
        "Subtypes".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<SubtypesConf>> = load_dir(&self.source_path, "sub")?;
        let mut tests = vec![];
        for tst in contents {}
        Ok(tests)
    }
}
