use language::languages::untyped_arithmetic::terms::Term;
use std::path::PathBuf;
use test_utils::{
    errors::Error,
    eval_test::EvalTest,
    latex_buss_test::LatexTestBuss,
    latex_frac_test::LatexTestFrac,
    load_tests::{load_dir, TestContents},
    parse_test::ParseTest,
    paths::{EXAMPLES_PATH, UNTYPED_ARITH_PATH},
    reparse_test::ReparseTest,
    setup,
    testsuite::{Test, TestSuite},
};
pub struct UntypedArithTests {
    source_dir: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct UntypedArithConf {
    expected: String,
}

impl UntypedArithTests {
    pub fn new(source_dir: PathBuf) -> UntypedArithTests {
        UntypedArithTests { source_dir }
    }
}

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = UntypedArithTests::new(examples_dir.join(UNTYPED_ARITH_PATH)).run_all()?;
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

impl TestSuite for UntypedArithTests {
    fn name(&self) -> String {
        "Untyped Arithmetic".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<UntypedArithConf>> = load_dir(&self.source_dir, "arith")?;
        let mut tests = vec![];
        for content in contents {
            let parse_test = ParseTest::<Term>::new(&content.source_name, &content.source_contents);
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test =
                ReparseTest::<Term>::new(&content.source_name, &content.source_contents);
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let eval_test = EvalTest::<Term>::new(
                &content.source_name,
                &content.source_contents,
                &content.conf.expected,
            );
            tests.push(Box::new(eval_test) as Box<dyn Test>);
            let latex_test =
                LatexTestBuss::<Term>::new(&content.source_name, &content.source_contents);
            tests.push(Box::new(latex_test) as Box<dyn Test>);
            let latex_test =
                LatexTestFrac::<Term>::new(&content.source_name, &content.source_contents);
            tests.push(Box::new(latex_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
