use super::TypecheckTest;
use std::path::PathBuf;
use test_common::{
    errors::Error,
    eval_test::EvalTest,
    load_tests::{load_dir, TestContents},
    parse_test::ParseTest,
    reparse_test::ReparseTest,
    testsuite::{Test, TestSuite},
};

pub struct FOmegaSubTests {
    source_dir: PathBuf,
}

impl FOmegaSubTests {
    pub fn new(source_dir: PathBuf) -> FOmegaSubTests {
        FOmegaSubTests { source_dir }
    }
}

#[derive(serde::Deserialize)]
pub struct FOmegaSubConf {
    ty: String,
    evaluated: String,
}

impl TestSuite for FOmegaSubTests {
    fn name(&self) -> String {
        "Existential".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<FOmegaSubConf>> = load_dir(&self.source_dir, "omega")?;
        let mut tests = vec![];
        for tst in contents {
            let parse_test = ParseTest::<f_omega_sub::syntax::terms::Term>::new(
                &tst.source_name,
                &tst.source_contents,
            );
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test = ReparseTest::<f_omega_sub::syntax::terms::Term>::new(
                &tst.source_name,
                &tst.source_contents,
            );
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let check_test =
                TypecheckTest::new(&tst.source_name, &tst.source_contents, &tst.conf.ty);
            tests.push(Box::new(check_test) as Box<dyn Test>);
            let eval_test = EvalTest::<f_omega_sub::syntax::terms::Term>::new(
                &tst.source_name,
                &tst.source_contents,
                &tst.conf.evaluated,
            );
            tests.push(Box::new(eval_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
