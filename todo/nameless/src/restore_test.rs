use nameless_representation::{remove_names::remove_names, restore_names::restore_names};
use test_common::testsuite::{Test, TestResult};
use untyped_lambda::parse::parse;

pub struct RestoreNamesTest {
    source_name: String,
    source_str: String,
    expected_restored: String,
}

impl RestoreNamesTest {
    pub fn new(source_name: &str, source_str: &str, expected: &str) -> RestoreNamesTest {
        RestoreNamesTest {
            source_name: source_name.to_owned(),
            source_str: source_str.to_owned(),
            expected_restored: expected.to_owned(),
        }
    }
}

impl Test for RestoreNamesTest {
    fn name(&self) -> String {
        format!("Restore Names {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        let mut src = self.source_str.clone();
        let parsed = match parse(&mut src) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let removed = remove_names(parsed.into());
        let result = restore_names(removed);
        TestResult::from_eq(&result, &self.expected_restored)
    }
}
