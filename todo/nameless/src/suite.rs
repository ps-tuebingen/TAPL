use super::{RemoveNamesTest, RemoveRestoreTest, RestoreNamesTest};
use std::path::PathBuf;
use test_common::{
    errors::Error,
    load_tests::{load_dir, TestContents},
    testsuite::{Test, TestSuite},
};

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

impl TestSuite for NamelessRepTests {
    fn name(&self) -> String {
        "Nameless Representation".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<NamelessConfig>> = load_dir(&self.source_dir, "lam")?;
        let mut tests = vec![];
        for source in contents {
            let remove_test = RemoveNamesTest::new(
                &source.source_name,
                &source.source_contents,
                &source.conf.nameless,
            );
            tests.push(Box::new(remove_test) as Box<dyn Test>);
            let restore_test = RestoreNamesTest::new(
                &source.source_name,
                &source.source_contents,
                &source.conf.restored,
            );
            tests.push(Box::new(restore_test) as Box<dyn Test>);
            let rem_res_test = RemoveRestoreTest::new(&source.source_name, &source.source_contents);
            tests.push(Box::new(rem_res_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
