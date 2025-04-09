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

impl TestSuite for ExceptionTests {
    fn name(&self) -> String {
        "Exceptions".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<ExceptionsConf>> = load_dir(&self.source_path, "ex")?;
        let mut tests = vec![];
        for tst in contents {
            let parse_test =
                ParseTest::<exceptions::syntax::Term>::new(&tst.source_name, &tst.source_contents);
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test = ReparseTest::<exceptions::syntax::Term>::new(
                &tst.source_name,
                &tst.source_contents,
            );
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let check_test = CheckTest::<exceptions::syntax::Term>::new(
                &tst.source_name,
                &tst.source_contents,
                &tst.conf.ty,
            );
            tests.push(Box::new(check_test) as Box<dyn Test>);
            let eval_test = EvalTest::<exceptions::syntax::Term>::new(
                &tst.source_name,
                &tst.source_contents,
                &tst.conf.evaluated,
            );
            tests.push(Box::new(eval_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
