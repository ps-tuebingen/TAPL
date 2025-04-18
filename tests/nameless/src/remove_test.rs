use nameless_representation::remove_names::remove_names;
use test_common::testsuite::{Test, TestResult};
use untyped_lambda::parse::parse;

pub struct RemoveNamesTest {
    pub source_name: String,
    pub named_source: String,
    pub expected_nameless: String,
}

impl RemoveNamesTest {
    pub fn new(source_name: &str, named_source: &str, expected: &str) -> RemoveNamesTest {
        RemoveNamesTest {
            source_name: source_name.to_owned(),
            named_source: named_source.to_owned(),
            expected_nameless: expected.to_owned(),
        }
    }
}

impl Test for RemoveNamesTest {
    fn name(&self) -> String {
        format!("Remove Names {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        let mut src = self.named_source.clone();
        let parsed = match parse(&mut src) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let removed = remove_names(parsed.clone().into());
        TestResult::from_eq(&removed, &self.expected_nameless)
    }
}
