use stlc::{eval_context::eval_with_context, parser::parse};
use test_common::testsuite::{Test, TestResult};

pub struct EvalCtxTest {
    source_name: String,
    source_contents: String,
    expected: String,
}

impl EvalCtxTest {
    pub fn new(source_name: &str, source_contents: &str, expected: &str) -> EvalCtxTest {
        EvalCtxTest {
            source_name: source_name.to_owned(),
            source_contents: source_contents.to_owned(),
            expected: expected.to_owned(),
        }
    }
}

impl Test for EvalCtxTest {
    fn name(&self) -> String {
        format!("Evaluating with Context {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        let parsed = match parse(self.source_contents.clone()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let evaled = match eval_with_context(parsed) {
            Ok(v) => v,
            Err(err) => return TestResult::from_err(err),
        };
        let result = evaled.to_string();
        TestResult::from_eq(&result, &self.expected)
    }
}
