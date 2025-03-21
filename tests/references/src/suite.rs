use std::path::PathBuf;
use test_common::{
    errors::Error,
    testsuite::{Test, TestSuite},
};

pub struct ReferencesTests {
    source_dir: PathBuf,
}

impl ReferencesTests {
    pub fn new(source_dir: PathBuf) -> ReferencesTests {
        ReferencesTests { source_dir }
    }
}

impl TestSuite for ReferencesTests {
    fn name(&self) -> String {
        "References".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        todo!()
    }
}
