use super::{
    test::{load_dir, Test, TestResult},
    TestRunner,
};
use std::path::PathBuf;
use stlc::parser::parse;

pub struct StlcTests {
    source_dir: PathBuf,
}

impl StlcTests {
    pub fn new(source_dir: PathBuf) -> StlcTests {
        StlcTests { source_dir }
    }
}

impl TestRunner for StlcTests {
    fn run_test(&self, test: Test) -> TestResult {
        match parse(test.source_str) {
            Ok(_) => TestResult::Success,
            Err(err) => TestResult::Fail(err.to_string()),
        }
    }

    fn load_tests(&self) -> Result<Vec<Test>, Box<dyn std::error::Error>> {
        load_dir(&self.source_dir)
    }
}
