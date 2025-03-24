use featherweight::parser::parse;
use test_common::testsuite::{Test, TestResult};

pub struct ReparseTest {
    source_name: String,
    source: String,
}

impl ReparseTest {
    pub fn new(source_name: &str, source: &str) -> ReparseTest {
        ReparseTest {
            source_name: source_name.to_owned(),
            source: source.to_owned(),
        }
    }
}

impl Test for ReparseTest {
    fn name(&self) -> String {
        format!("Reparsing {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        let parsed = match parse(self.source.clone()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let reparsed = match parse(parsed.to_string()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        TestResult::from_eq(&reparsed, &parsed)
    }
}
