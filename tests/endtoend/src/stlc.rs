use super::{
    errors::Error,
    test::{load_dir, Test, TestResult},
    TestRunner,
};
use std::path::PathBuf;
use stlc::parser::parse;

pub struct StlcTests {
    source_dir: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct StlcConf {
    expected: String,
}

impl StlcTests {
    pub fn new(source_dir: PathBuf) -> StlcTests {
        StlcTests { source_dir }
    }
}

impl TestRunner for StlcTests {
    type TestConf = StlcConf;

    fn run_test(&self, test: Test<Self::TestConf>) -> TestResult {
        match parse(test.source_str) {
            Ok(_) => TestResult::Success,
            Err(err) => TestResult::Fail(err.to_string()),
        }
    }

    fn load_tests(&self) -> Result<Vec<Test<Self::TestConf>>, Error> {
        load_dir(&self.source_dir, "stlc")
    }
}
