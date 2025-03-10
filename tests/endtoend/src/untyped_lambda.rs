use super::{
    test::{load_dir, Test, TestResult},
    TestRunner,
};
use std::path::PathBuf;
use untyped_lambda::parse::parse;

pub struct UntypedLambdaTests {
    source_dir: PathBuf,
}

impl UntypedLambdaTests {
    pub fn new(source_dir: PathBuf) -> UntypedLambdaTests {
        UntypedLambdaTests { source_dir }
    }
}

impl TestRunner for UntypedLambdaTests {
    fn suite_name(&self) -> String {
        "Untyped Lambda".to_owned()
    }

    fn run_test(&self, test: Test) -> TestResult {
        let mut source = test.source_str;
        match parse(&mut source) {
            Ok(_) => TestResult::Success,
            Err(err) => TestResult::Fail(err.to_string()),
        }
    }

    fn load_tests(&self) -> Result<Vec<Test>, Box<dyn std::error::Error>> {
        load_dir(&self.source_dir)
    }
}
