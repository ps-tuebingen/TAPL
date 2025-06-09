use language::languages::untyped_lambda::terms::Term;
use std::path::PathBuf;
use test_utils::{
    errors::Error,
    eval_test::EvalTest,
    latex_test::LatexTest,
    load_tests::{load_dir, TestContents},
    parse_test::ParseTest,
    paths::{EXAMPLES_PATH, UNTYPED_LAMBDA_PATH},
    reparse_test::ReparseTest,
    setup,
    testsuite::{Test, TestSuite},
};
pub struct UntypedLambdaTests {
    source_dir: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct UntypedLambdaConf {
    evaluated: String,
}

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = UntypedLambdaTests::new(examples_dir.join(UNTYPED_LAMBDA_PATH)).run_all()?;

    println!(
        "Finished running tests with \x1b[31m{} fails\x1b[39m",
        fails
    );
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}

impl UntypedLambdaTests {
    pub fn new(source_dir: PathBuf) -> UntypedLambdaTests {
        UntypedLambdaTests { source_dir }
    }
}

impl TestSuite for UntypedLambdaTests {
    fn name(&self) -> String {
        "Untyped Lambda".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<UntypedLambdaConf>> = load_dir(&self.source_dir, "lam")?;
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
                &content.conf.evaluated,
            );
            tests.push(Box::new(eval_test) as Box<dyn Test>);
            let latex_test = LatexTest::<Term>::new(&content.source_name, &content.source_contents);
            tests.push(Box::new(latex_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
