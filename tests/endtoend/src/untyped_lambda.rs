use super::{
    errors::Error,
    test::{load_dir, Test, TestResult},
    TestRunner,
};
use std::path::PathBuf;
use untyped_lambda::parse::parse;

pub struct UntypedLambdaTests {
    source_dir: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct UntypedLambdaConf {
    expected: String,
}

impl UntypedLambdaTests {
    pub fn new(source_dir: PathBuf) -> UntypedLambdaTests {
        UntypedLambdaTests { source_dir }
    }
}

impl TestRunner for UntypedLambdaTests {
    type TestConf = UntypedLambdaConf;

    fn run_test(&self, test: Test<Self::TestConf>) -> TestResult {
        let mut source = test.source_str;
        match parse(&mut source) {
            Ok(_) => TestResult::Success,
            Err(err) => TestResult::Fail(err.to_string()),
        }
    }

    fn load_tests(&self) -> Result<Vec<Test<Self::TestConf>>, Error> {
        load_dir(&self.source_dir, "lam")
    }
}
