use common::errors::Error;
use std::path::PathBuf;
use test_utils::{
    check_test::CheckTest,
    eval_test::EvalTest,
    load_tests::{load_dir, TestContents},
    parse_test::ParseTest,
    paths::{EXAMPLES_PATH, F_OMEGA_SUB_PATH},
    reparse_test::ReparseTest,
    setup,
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
fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = FOmegaSubTests::new(examples_dir.join(F_OMEGA_SUB_PATH)).run_all()?;

    println!(
        "Finished running tests with \x1b[31m{} fails\x1b[39m",
        fails
    );
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}

impl TestSuite for FOmegaSubTests {
    fn name(&self) -> String {
        "Existential".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<FOmegaSubConf>> = load_dir(&self.source_dir, "omega")?;
        let mut tests = vec![];
        for tst in contents {
            let parse_test = ParseTest::<languages::f_omega_sub::terms::Term>::new(
                &tst.source_name,
                &tst.source_contents,
            );
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test = ReparseTest::<languages::f_omega_sub::terms::Term>::new(
                &tst.source_name,
                &tst.source_contents,
            );
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let check_test = CheckTest::<languages::f_omega_sub::terms::Term>::new(
                &tst.source_name,
                &tst.source_contents,
                &tst.conf.ty,
            );
            tests.push(Box::new(check_test) as Box<dyn Test>);
            let eval_test = EvalTest::<languages::f_omega_sub::terms::Term>::new(
                &tst.source_name,
                &tst.source_contents,
                &tst.conf.evaluated,
            );
            tests.push(Box::new(eval_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
