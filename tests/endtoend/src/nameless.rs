use super::{
    errors::Error,
    test::{load_dir, TestContents},
    testsuite::{Test, TestResult, TestSuite},
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

#[derive(serde::Deserialize)]
pub struct NamelessConfig {
    nameless: String,
    restored: String,
}

pub struct RemoveNamesTest {
    pub source_name: String,
    pub named_source: String,
    pub expected_nameless: String,
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

pub struct RestoreNamesTest {
    source_name: String,
    source_str: String,
    expected_restored: String,
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

pub struct RemoveRestoreTest {
    source_name: String,
    named_source: String,
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

impl TestSuite for NamelessRepTests {
    fn name(&self) -> String {
        "Nameless Representation".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<NamelessConfig>> = load_dir(&self.source_dir, "lam")?;
        let mut tests = vec![];
        for source in contents {
            let remove_test = RemoveNamesTest {
                source_name: source.source_name.clone(),
                named_source: source.source_contents.clone(),
                expected_nameless: source.conf.nameless,
            };
            tests.push(Box::new(remove_test) as Box<dyn Test>);
            let restore_test = RestoreNamesTest {
                source_name: source.source_name.clone(),
                source_str: source.source_contents.clone(),
                expected_restored: source.conf.restored,
            };
            tests.push(Box::new(restore_test) as Box<dyn Test>);
            let rem_res_test = RemoveRestoreTest {
                source_name: source.source_name,
                named_source: source.source_contents,
            };
            tests.push(Box::new(rem_res_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
