use super::{
    errors::Error,
    test::{load_dir, Test, TestResult},
    TestRunner,
};
use std::path::PathBuf;
use untyped_lambda::{
    eval::{eval, EvalOrder},
    parse::parse,
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

impl TestRunner for UntypedLambdaTests {
    type TestConf = UntypedLambdaConf;

    fn run_test(&self, test: Test<Self::TestConf>) -> TestResult {
        let mut source = test.source_str;
        let parsed = match parse(&mut source) {
            Ok(p) => p,
            Err(err) => return TestResult::Fail(err.to_string()),
        };
        let reparsed = match parse(&mut parsed.to_string()) {
            Ok(t) => t,
            Err(err) => return TestResult::Fail(format!("Could not reparse parsed term:\n{err}")),
        };
        if parsed != reparsed {
            return TestResult::Fail(format!(
                "Reparsed does not equal parsed:\n\tresult:   {reparsed:?}\n\texpected: {parsed:?}"
            ));
        }
        let evaluated = eval(parsed, EvalOrder::CBV);
        let result = evaluated.to_string();
        let expected = test.config.evaluated;
        if result == expected {
            TestResult::Success
        } else {
            TestResult::Fail(format!(
                "Evaluated does not match expected:\n\tresult:  {result}\n\texpected:{expected}"
            ))
        }
    }

    fn load_tests(&self) -> Result<Vec<Test<Self::TestConf>>, Error> {
        load_dir(&self.source_dir, "lam")
    }
}
