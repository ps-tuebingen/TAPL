use super::{
    errors::Error,
    test::{load_dir, Test, TestResult},
    TestRunner,
};
use std::path::PathBuf;
use untyped_arithmetic::parse::parse;

pub struct UntypedArithTests {
    source_dir: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct UntypedArithConf {
    expected: String,
}

impl UntypedArithTests {
    pub fn new(source_dir: PathBuf) -> UntypedArithTests {
        UntypedArithTests { source_dir }
    }
}

impl TestRunner for UntypedArithTests {
    type TestConf = UntypedArithConf;

    fn run_test(&self, test: Test<Self::TestConf>) -> TestResult {
        let parsed = match parse(test.source_str) {
            Ok(t) => t,
            Err(err) => return TestResult::Fail(err.to_string()),
        };
        let reparsed = match parse(parsed.to_string()) {
            Ok(t) => t,
            Err(err) => return TestResult::Fail(format!("Could not reparse formatted:\n{err}")),
        };
        if reparsed != parsed {
            return TestResult::Fail(format!(
                "Reparsed does not match parsed:\nresult:   {reparsed}\n\texpected: {parsed}"
            ));
        }
        let evaled = parsed.eval();
        let result = evaled.to_string();
        let expected = test.config.expected;
        if result != expected {
            TestResult::Fail(format!(
                "Result != Expected:\n\tresult:   {result}\n\texpected: {expected}"
            ))
        } else {
            TestResult::Success
        }
    }

    fn load_tests(&self) -> Result<Vec<Test<Self::TestConf>>, Error> {
        load_dir(&self.source_dir, "arith")
    }
}
