use common::Eval;
use test_common::testsuite::{Test, TestResult};
use untyped_lambda::parse::parse;

pub struct EvalTest {
    source_name: String,
    source_contents: String,
    expected: String,
}

impl EvalTest {
    pub fn new(source_name: &str, source: &str, expected: &str) -> EvalTest {
        EvalTest {
            source_name: source_name.to_owned(),
            source_contents: source.to_owned(),
            expected: expected.to_owned(),
        }
    }
}

impl Test for EvalTest {
    fn name(&self) -> String {
        format!("Evaluate {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        let mut src = self.source_contents.clone();
        let parsed = match parse(&mut src) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let evaled = parsed.eval(Default::default()).unwrap();
        TestResult::from_eq(&evaled, &self.expected)
    }
}
