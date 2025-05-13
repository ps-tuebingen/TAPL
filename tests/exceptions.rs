use common::errors::Error;
use e2e_common::{
    check_test::CheckTest,
    eval_test::EvalTest,
    load_tests::{load_dir, TestContents},
    parse_test::ParseTest,
    paths::{EXAMPLES_PATH, EXCEPTIONS_PATH},
    reparse_test::ReparseTest,
    setup,
    testsuite::{Test, TestSuite},
};
use std::path::PathBuf;

pub struct ExceptionTests {
    source_path: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct ExceptionsConf {
    ty: String,
    evaluated: String,
}

impl ExceptionTests {
    pub fn new(path: PathBuf) -> ExceptionTests {
        ExceptionTests { source_path: path }
    }
}

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = ExceptionTests::new(examples_dir.join(EXCEPTIONS_PATH)).run_all()?;

    println!(
        "Finished running tests with \x1b[31m{} fails\x1b[39m",
        fails
    );
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}

impl TestSuite for ExceptionTests {
    fn name(&self) -> String {
        "Exceptions".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<ExceptionsConf>> = load_dir(&self.source_path, "ex")?;
        let mut tests = vec![];
        for tst in contents {
            let parse_test = ParseTest::<languages::exceptions::terms::Term>::new(
                &tst.source_name,
                &tst.source_contents,
            );
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test = ReparseTest::<languages::exceptions::terms::Term>::new(
                &tst.source_name,
                &tst.source_contents,
            );
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let check_test = CheckTest::<languages::exceptions::terms::Term>::new(
                &tst.source_name,
                &tst.source_contents,
                &tst.conf.ty,
            );
            tests.push(Box::new(check_test) as Box<dyn Test>);
            let eval_test = EvalTest::<languages::exceptions::terms::Term>::new(
                &tst.source_name,
                &tst.source_contents,
                &tst.conf.evaluated,
            );
            tests.push(Box::new(eval_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
