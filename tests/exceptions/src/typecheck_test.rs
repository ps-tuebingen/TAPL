use common::Typecheck;
use exceptions::parser::parse;
use test_common::testsuite::{Test, TestResult};

pub struct TypecheckTest {
    source_name: String,
    source: String,
    expected: String,
}

impl TypecheckTest {
    pub fn new(source_name: &str, source: &str, expected: &str) -> TypecheckTest {
        TypecheckTest {
            source_name: source_name.to_owned(),
            source: source.to_owned(),
            expected: expected.to_owned(),
        }
    }
}

impl Test for TypecheckTest {
    fn name(&self) -> String {
        format!("Type checking {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        let parsed = match parse(self.source.clone()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let checked = match parsed.check(&mut Default::default()) {
            Ok(ty) => ty,
            Err(err) => return TestResult::from_err(err),
        };
        TestResult::from_eq(&checked, &self.expected)
    }
}
