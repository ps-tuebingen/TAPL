use super::{ParseTest, ReparseTest};
use std::path::PathBuf;
use test_common::{
    errors::Error,
    load_tests::{load_dir, TestContents},
    testsuite::{Test, TestSuite},
};

pub struct BoundedTests {
    source_dir: PathBuf,
}

impl BoundedTests {
    pub fn new(source_dir: PathBuf) -> BoundedTests {
        BoundedTests { source_dir }
    }
}

#[derive(serde::Deserialize)]
pub struct NamelessConfig {}

impl TestSuite for BoundedTests {
    fn name(&self) -> String {
        "Bounded Quantification".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<NamelessConfig>> = load_dir(&self.source_dir, "bd")?;
        let mut tests = vec![];
        for tst in contents {
            let parse_test = ParseTest::new(&tst.source_name, &tst.source_contents);
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test = ReparseTest::new(&tst.source_name, &tst.source_contents);
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
