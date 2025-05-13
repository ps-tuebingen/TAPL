use std::path::PathBuf;
use test_common::{
    errors::Error,
    eval_test::EvalTest,
    load_tests::{load_dir, TestContents},
    parse_test::ParseTest,
    reparse_test::ReparseTest,
    testsuite::{Test, TestSuite},
};

pub struct UntypedLambdaTests {
    source_dir: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct UntypedLambdaConf {
    evaluated: String,
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
            let parse_test = ParseTest::<languages::untyped_lambda::terms::Term>::new(
                &content.source_name,
                &content.source_contents,
            );
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test = ReparseTest::<languages::untyped_lambda::terms::Term>::new(
                &content.source_name,
                &content.source_contents,
            );
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let eval_test = EvalTest::<languages::untyped_lambda::terms::Term>::new(
                &content.source_name,
                &content.source_contents,
                &content.conf.evaluated,
            );
            tests.push(Box::new(eval_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
