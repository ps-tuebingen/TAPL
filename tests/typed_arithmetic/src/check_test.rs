use test_common::testsuite::{Test, TestResult};
use typed_arithmetic::{check::check, parser::parse};

pub struct TypecheckTest {
    source_name: String,
    source_contents: String,
    expected: String,
}

impl TypecheckTest {
    pub fn new(source_name: &str, source_contents: &str, expected: &str) -> TypecheckTest {
        TypecheckTest {
            source_name: source_name.to_owned(),
            source_contents: source_contents.to_owned(),
            expected: expected.to_owned(),
        }
    }
}

impl Test for TypecheckTest {
    fn name(&self) -> String {
        format!("Typechecking {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        let parsed = match parse(self.source_contents.clone()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let checked = match check(&parsed) {
            Ok(ty) => ty,
            Err(err) => return TestResult::from_err(err),
        };
        TestResult::from_eq(&checked, &self.expected)
    }
}
