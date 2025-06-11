use language::languages::system_f::terms::Term;
use std::path::PathBuf;
use test_utils::{
    check_test::CheckTest,
    errors::Error,
    eval_test::EvalTest,
    latex_buss_test::LatexTestBuss,
    latex_frac_test::LatexTestFrac,
    load_tests::{load_dir, TestContents},
    parse_test::ParseTest,
    paths::{EXAMPLES_PATH, SYSTEMF_PATH},
    reparse_test::ReparseTest,
    setup,
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

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = SystemFTests::new(examples_dir.join(SYSTEMF_PATH)).run_all()?;
    let fail_str = if fails > 0 {
        format!("\x1b[31m{fails} fails\x1b[39m")
    } else {
        "0 fails".to_owned()
    };
    println!("Finished running tests with {fail_str}");
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}

impl TestSuite for SystemFTests {
    fn name(&self) -> String {
        "System F".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<SystemFConf>> = load_dir(&self.source_path, "f")?;
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
        }
        Ok(tests)
    }
}
