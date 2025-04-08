use bounded_quantification::parser::parse;
use common::Eval;
use test_common::testsuite::{Test, TestResult};

pub struct EvalTest {
    source_name: String,
    source: String,
    expected: String,
}

impl EvalTest {
    pub fn new(source_name: &str, source: &str, expected: &str) -> EvalTest {
        EvalTest {
            source_name: source_name.to_owned(),
            source: source.to_owned(),
            expected: expected.to_owned(),
        }
    }
}

impl Test for EvalTest {
    fn name(&self) -> String {
        format!("Evaluating {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        let parsed = match parse(self.source.clone()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let evaled = match parsed.eval(Default::default()) {
            Ok(v) => v,
            Err(err) => return TestResult::from_err(err),
        };
        TestResult::from_eq(&evaled, &self.expected)
    }
}
