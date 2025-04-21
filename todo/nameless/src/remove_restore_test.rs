use nameless_representation::{remove_names::remove_names, restore_names::restore_names};
use test_common::testsuite::{Test, TestResult};
use untyped_lambda::parse::parse;

pub struct RemoveRestoreTest {
    source_name: String,
    named_source: String,
}

impl RemoveRestoreTest {
    pub fn new(source_name: &str, named_source: &str) -> RemoveRestoreTest {
        RemoveRestoreTest {
            source_name: source_name.to_owned(),
            named_source: named_source.to_owned(),
        }
    }
}

impl Test for RemoveRestoreTest {
    fn name(&self) -> String {
        format!("Remove and Restore names {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        let mut src = self.named_source.clone();
        let parsed = match parse(&mut src) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let removed = remove_names(parsed.clone().into());
        let restored = restore_names(removed.clone());
        let removed_again = remove_names(restored);
        TestResult::from_eq(&removed_again, &removed)
    }
}
