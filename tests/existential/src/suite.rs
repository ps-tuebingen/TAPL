use super::{/*EvalTest, */ ParseTest, ReparseTest, TypecheckTest};
use std::path::PathBuf;
use test_common::{
    errors::Error,
    load_tests::{load_dir, TestContents},
    testsuite::{Test, TestSuite},
};

pub struct ExistentialTests {
    source_dir: PathBuf,
}

impl ExistentialTests {
    pub fn new(source_dir: PathBuf) -> ExistentialTests {
        ExistentialTests { source_dir }
    }
}

#[derive(serde::Deserialize)]
pub struct ExistentialConf {
    ty: String,
    evaluated: String,
}

impl TestSuite for ExistentialTests {
    fn name(&self) -> String {
        "Existential".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<ExistentialConf>> = load_dir(&self.source_dir, "ex")?;
        let mut tests = vec![];
        for tst in contents {
            let parse_test = ParseTest::new(&tst.source_name, &tst.source_contents);
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test = ReparseTest::new(&tst.source_name, &tst.source_contents);
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let check_test =
                TypecheckTest::new(&tst.source_name, &tst.source_contents, &tst.conf.ty);
            tests.push(Box::new(check_test) as Box<dyn Test>);
            /*let eval_test =
                EvalTest::new(&tst.source_name, &tst.source_contents, &tst.conf.evaluated);
            tests.push(Box::new(eval_test) as Box<dyn Test>);*/
        }
        Ok(tests)
    }
}
