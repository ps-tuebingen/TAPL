use exceptions::parser::parse;
use test_common::testsuite::{Test, TestResult};

pub struct ParseTest {
    source_name: String,
    source: String,
}

impl ParseTest {
    pub fn new(source_name: &str, source: &str) -> ParseTest {
        ParseTest {
            source_name: source_name.to_owned(),
            source: source.to_owned(),
        }
    }
}

impl Test for ParseTest {
    fn name(&self) -> String {
        format!("Parsing {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        match parse(self.source.clone()) {
            Ok(_) => TestResult::Success,
            Err(err) => TestResult::from_err(err),
        }
    }
}
