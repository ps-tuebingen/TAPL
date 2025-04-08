use test_common::testsuite::{Test, TestResult};
use untyped_lambda::parse::parse;

pub struct ParseTest {
    source_name: String,
    source_contents: String,
}

impl ParseTest {
    pub fn new(source_name: &str, source: &str) -> ParseTest {
        ParseTest {
            source_name: source_name.to_owned(),
            source_contents: source.to_owned(),
        }
    }
}

impl Test for ParseTest {
    fn name(&self) -> String {
        format!("Parse {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        let mut src = self.source_contents.clone();
        match parse(&mut src) {
            Ok(_) => TestResult::Success,
            Err(err) => TestResult::from_err(err),
        }
    }
}
