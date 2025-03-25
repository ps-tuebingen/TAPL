use inference::{bidirectional::Infer, parser::parse};
use test_common::testsuite::{Test, TestResult};

pub struct BidirectionalTest {
    source_name: String,
    source: String,
    expected: String,
}

impl BidirectionalTest {
    pub fn new(source_name: &str, source: &str, expected: &str) -> BidirectionalTest {
        BidirectionalTest {
            source_name: source_name.to_owned(),
            source: source.to_owned(),
            expected: expected.to_owned(),
        }
    }
}

impl Test for BidirectionalTest {
    fn name(&self) -> String {
        format!("Bidirectional Inference for {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        let parsed = match parse(self.source.clone()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let inferred = match parsed.infer(&mut Default::default()) {
            Ok(ty) => ty,
            Err(err) => return TestResult::from_err(err),
        };
        TestResult::from_eq(&inferred, &self.expected)
    }
}
