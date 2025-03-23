use super::{ParseTest, ReparseTest, TypecheckTest};
use std::path::PathBuf;
use test_common::{
    errors::Error,
    load_tests::{load_dir, TestContents},
    testsuite::{Test, TestSuite},
};

pub struct ReferencesTests {
    source_dir: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct ReferencesConf {
    ty: String,
    evaluated: String,
}

impl ReferencesTests {
    pub fn new(source_dir: PathBuf) -> ReferencesTests {
        ReferencesTests { source_dir }
    }
}

impl TestSuite for ReferencesTests {
    fn name(&self) -> String {
        "References".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<ReferencesConf>> = load_dir(&self.source_dir, "ref")?;
        let mut tests = vec![];
        for tst in contents {
            let parse_test = ParseTest::new(&tst.source_name, &tst.source_contents);
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test = ReparseTest::new(&tst.source_name, &tst.source_contents);
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let check_test =
                TypecheckTest::new(&tst.source_name, &tst.source_contents, &tst.conf.ty);
            tests.push(Box::new(check_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
