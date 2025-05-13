use common::errors::Error;
use e2e_common::{
    check_test::CheckTest,
    eval_test::EvalTest,
    load_tests::{load_dir, TestContents},
    parse_test::ParseTest,
    paths::{EXAMPLES_PATH, LAMBDA_OMEGA_PATH},
    reparse_test::ReparseTest,
    setup,
    testsuite::{Test, TestSuite},
};
use std::path::PathBuf;

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

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = LambdaOmegaTests::new(examples_dir.join(LAMBDA_OMEGA_PATH)).run_all()?;

    println!(
        "Finished running tests with \x1b[31m{} fails\x1b[39m",
        fails
    );
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
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
