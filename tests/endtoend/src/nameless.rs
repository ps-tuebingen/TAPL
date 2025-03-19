use super::{
    errors::Error,
    test::{load_dir, Test, TestResult},
    TestRunner,
};
use nameless_representation::{remove_names::remove_names, restore_names::restore_names};
use std::path::PathBuf;
use untyped_lambda::parse::parse;

pub struct NamelessRepTests {
    source_dir: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct NamelessConfig {
    nameless: String,
    restored: String,
}

impl NamelessRepTests {
    pub fn new(source_dir: PathBuf) -> NamelessRepTests {
        NamelessRepTests { source_dir }
    }
}

impl TestRunner for NamelessRepTests {
    type TestConf = NamelessConfig;
    fn run_test(&self, test: Test<Self::TestConf>) -> TestResult {
        let mut source = test.source_str;
        let parsed = match parse(&mut source) {
            Ok(p) => p,
            Err(err) => return TestResult::Fail(err.to_string()),
        };
        let nameless = remove_names(parsed.into());
        let result = nameless.to_string();
        let expected = test.config.nameless;
        if result != expected {
            return TestResult::Fail(format!(
                "Nameless Representation does not match expected:\nresult:{result},expected:{expected}",
            ));
        }
        let named = restore_names(nameless.clone());
        let result = named.to_string();
        let expected = test.config.restored;
        if result != expected {
            return TestResult::Fail(format!(
                "Restored Name does not match expected:\nresult:{result},expected:{expected}"
            ));
        }

        TestResult::Success
    }

    fn load_tests(&self) -> Result<Vec<Test<Self::TestConf>>, Error> {
        load_dir(&self.source_dir, "lam")
    }
}
