use f_omega_sub::parser::parse;
use test_common::testsuite::{Test, TestResult};

pub struct ReparseTest {
    source_name: String,
    source_contents: String,
}

impl ReparseTest {
    pub fn new(source_name: &str, source_contents: &str) -> ReparseTest {
        ReparseTest {
            source_name: source_name.to_owned(),
            source_contents: source_contents.to_owned(),
        }
    }
}

impl Test for ReparseTest {
    fn name(&self) -> String {
        format!("Reparsing {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        let parsed = match parse(self.source_contents.clone()) {
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
