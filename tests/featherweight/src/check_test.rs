use featherweight::{parser::parse, typing::class::class_ok};
use test_common::testsuite::{Test, TestResult};

pub struct TypecheckTest {
    source_name: String,
    source: String,
}

impl TypecheckTest {
    pub fn new(source_name: &str, source: &str) -> TypecheckTest {
        TypecheckTest {
            source_name: source_name.to_owned(),
            source: source.to_owned(),
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
        for decl in parsed.classes.clone().into_values() {
            let name = decl.name.clone();
            let checked = class_ok(decl, &parsed);
            if !checked {
                return TestResult::Fail(format!("Class {name} cannot be checked"));
            }
        }
        TestResult::Success
    }
}
