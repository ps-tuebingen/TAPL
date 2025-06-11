use language::languages::exceptions::terms::Term;
use std::path::PathBuf;
use test_utils::{
    check_test::CheckTest,
    errors::Error,
    eval_test::EvalTest,
    latex_buss_test::LatexTestBuss,
    latex_frac_test::LatexTestFrac,
    latex_trace_test::LatexTestTrace,
    load_tests::{load_dir, TestContents},
    parse_test::ParseTest,
    paths::{EXAMPLES_PATH, EXCEPTIONS_PATH},
    reparse_test::ReparseTest,
    setup,
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

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = ExceptionTests::new(examples_dir.join(EXCEPTIONS_PATH)).run_all()?;

    let fail_str = if fails > 0 {
        format!("\x1b[31m{fails} fails\x1b[39m")
    } else {
        "0 fails".to_owned()
    };

    println!("Finished running tests with {fail_str}",);
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
            let parse_test = ParseTest::<Term>::new(&tst.source_name, &tst.source_contents);
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test = ReparseTest::<Term>::new(&tst.source_name, &tst.source_contents);
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let check_test =
                CheckTest::<Term>::new(&tst.source_name, &tst.source_contents, &tst.conf.ty);
            tests.push(Box::new(check_test) as Box<dyn Test>);
            let eval_test =
                EvalTest::<Term>::new(&tst.source_name, &tst.source_contents, &tst.conf.evaluated);
            tests.push(Box::new(eval_test) as Box<dyn Test>);
            let latex_test = LatexTestBuss::<Term>::new(&tst.source_name, &tst.source_contents);
            tests.push(Box::new(latex_test) as Box<dyn Test>);
            let latex_test = LatexTestFrac::<Term>::new(&tst.source_name, &tst.source_contents);
            tests.push(Box::new(latex_test) as Box<dyn Test>);
            let latex_test = LatexTestTrace::<Term>::new(&tst.source_name, &tst.source_contents);
            tests.push(Box::new(latex_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
