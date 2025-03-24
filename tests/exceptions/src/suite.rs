use std::path::PathBuf;
use test_common::{
    errors::Error,
    load_tests::{load_dir, TestContents},
    testsuite::{Test, TestSuite},
};

pub struct ExceptionTests {
    source_path: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct ExceptionsConf {
    ty: String,
    evaluated: String,
}

impl ExceptionTests {
    pub fn new(path: PathBuf) -> ExceptionTests {
        ExceptionTests { source_path: path }
    }
}

impl TestSuite for ExceptionTests {
    fn name(&self) -> String {
        "Exceptions".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<ExceptionsConf>> = load_dir(&self.source_path, "ex")?;
        let mut tests = vec![];
        for tst in contents {}
        Ok(tests)
    }
}
