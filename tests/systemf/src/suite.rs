use super::{EvalTest, ParseTest, ReparseTest, TypecheckTest};
use std::path::PathBuf;
use test_common::{
    errors::Error,
    load_tests::{load_dir, TestContents},
    testsuite::{Test, TestSuite},
};

pub struct SystemFTests {
    source_path: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct SystemFConf {
    ty: String,
    evaluated: String,
}

impl SystemFTests {
    pub fn new(path: PathBuf) -> SystemFTests {
        SystemFTests { source_path: path }
    }
}

impl TestSuite for SystemFTests {
    fn name(&self) -> String {
        "System F".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<SystemFConf>> = load_dir(&self.source_path, "f")?;
        let mut tests = vec![];
        for tst in contents {
            let parse_test = ParseTest::new(&tst.source_name, &tst.source_contents);
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test = ReparseTest::new(&tst.source_name, &tst.source_contents);
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let check_test =
                TypecheckTest::new(&tst.source_name, &tst.source_contents, &tst.conf.ty);
            tests.push(Box::new(check_test) as Box<dyn Test>);
            let eval_test =
                EvalTest::new(&tst.source_name, &tst.source_contents, &tst.conf.evaluated);
            tests.push(Box::new(eval_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
