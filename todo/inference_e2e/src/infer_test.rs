use inference::{constraints::typecheck, parser::parse};
use test_common::testsuite::{Test, TestResult};

pub struct InferTest {
    source_name: String,
    source: String,
    expected: String,
}

impl InferTest {
    pub fn new(source_name: &str, source: &str, expected: &str) -> InferTest {
        InferTest {
            source_name: source_name.to_owned(),
            source: source.to_owned(),
            expected: expected.to_owned(),
        }
    }
}

impl Test for InferTest {
    fn name(&self) -> String {
        format!("Type Inference for {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        let parsed = match parse(self.source.clone()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let inferred = match typecheck(parsed) {
            Ok(ty) => ty,
            Err(err) => return TestResult::from_err(err),
        };
        TestResult::from_eq(&inferred, &self.expected)
    }
}
