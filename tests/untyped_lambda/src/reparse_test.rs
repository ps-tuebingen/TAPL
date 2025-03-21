use test_common::testsuite::{Test, TestResult};
use untyped_lambda::parse::parse;

pub struct ReparseTest {
    source_name: String,
    source_contents: String,
}

impl ReparseTest {
    pub fn new(source_name: &str, source: &str) -> ReparseTest {
        ReparseTest {
            source_name: source_name.to_owned(),
            source_contents: source.to_owned(),
        }
    }
}

impl Test for ReparseTest {
    fn name(&self) -> String {
        format!("Reparse {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        let mut src = self.source_contents.clone();
        let parsed = match parse(&mut src) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let reparsed = match parse(&mut parsed.to_string()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        TestResult::from_eq(&reparsed, &parsed)
    }
}
