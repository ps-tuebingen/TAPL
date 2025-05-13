use std::path::PathBuf;
use test_common::{
    check_test::CheckTest,
    errors::Error,
    eval_test::EvalTest,
    load_tests::{load_dir, TestContents},
    parse_test::ParseTest,
    reparse_test::ReparseTest,
    testsuite::{Test, TestSuite},
};

pub struct LambdaOmegaTests {
    source_dir: PathBuf,
}

impl LambdaOmegaTests {
    pub fn new(source_dir: PathBuf) -> LambdaOmegaTests {
        LambdaOmegaTests { source_dir }
    }
}

#[derive(serde::Deserialize)]
pub struct BoundedConf {
    ty: String,
    evaluated: String,
}

impl TestSuite for LambdaOmegaTests {
    fn name(&self) -> String {
        "Lambda Omega".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<BoundedConf>> = load_dir(&self.source_dir, "lamo")?;
        let mut tests = vec![];
        for tst in contents {
            let parse_test = ParseTest::<languages::lambda_omega::terms::Term>::new(
                &tst.source_name,
                &tst.source_contents,
            );
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test = ReparseTest::<languages::lambda_omega::terms::Term>::new(
                &tst.source_name,
                &tst.source_contents,
            );
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let check_test = CheckTest::<languages::lambda_omega::terms::Term>::new(
                &tst.source_name,
                &tst.source_contents,
                &tst.conf.ty,
            );
            tests.push(Box::new(check_test) as Box<dyn Test>);
            let eval_test = EvalTest::<languages::lambda_omega::terms::Term>::new(
                &tst.source_name,
                &tst.source_contents,
                &tst.conf.evaluated,
            );
            tests.push(Box::new(eval_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
