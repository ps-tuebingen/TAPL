use super::{
    test::{load_dir, Test, TestResult},
    TestRunner,
};
use nameless_representation::{remove_names::remove_names, restore_names::restore_names};
use std::path::PathBuf;
use untyped_lambda::parse::parse;

pub struct NamelessRepTests {
    source_dir: PathBuf,
}

impl NamelessRepTests {
    pub fn new(source_dir: PathBuf) -> NamelessRepTests {
        NamelessRepTests { source_dir }
    }
}

impl TestRunner for NamelessRepTests {
    fn run_test(&self, test: Test) -> TestResult {
        let mut source = test.source_str;
        let parsed = match parse(&mut source) {
            Ok(p) => p,
            Err(err) => return TestResult::Fail(err.to_string()),
        };
        let nameless = remove_names(parsed.into());
        let named = restore_names(nameless.clone());
        let nameless2 = remove_names(named);
        if nameless2 == nameless {
            TestResult::Success
        } else {
            TestResult::Fail(format!(
                "Different results removing and restoring names: {nameless}, {nameless2}",
            ))
        }
    }

    fn load_tests(&self) -> Result<Vec<Test>, Box<dyn std::error::Error>> {
        load_dir(&self.source_dir)
    }
}
