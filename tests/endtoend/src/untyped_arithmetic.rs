use super::{
    test::{load_dir, Test, TestResult},
    TestRunner,
};
use std::path::PathBuf;
use untyped_arithmetic::parse::parse;

pub struct UntypedArithTests {
    source_dir: PathBuf,
}

impl UntypedArithTests {
    pub fn new(source_dir: PathBuf) -> UntypedArithTests {
        UntypedArithTests { source_dir }
    }
}

impl TestRunner for UntypedArithTests {
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
