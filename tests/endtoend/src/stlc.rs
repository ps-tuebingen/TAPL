use super::{
    errors::Error,
    test::{load_dir, Test, TestResult},
    TestRunner,
};
use std::path::PathBuf;
use stlc::{check::Check, eval::Eval, parser::parse};

pub struct StlcTests {
    source_dir: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct StlcConf {
    ty: String,
    evaled: String,
}

impl StlcTests {
    pub fn new(source_dir: PathBuf) -> StlcTests {
        StlcTests { source_dir }
    }
}

impl TestRunner for StlcTests {
    type TestConf = StlcConf;

    fn run_test(&self, test: Test<Self::TestConf>) -> TestResult {
        let parsed = match parse(test.source_str) {
            Ok(t) => t,
            Err(err) => return TestResult::Fail(err.to_string()),
        };
        let reparsed = match parse(parsed.to_string()) {
            Ok(t) => t,
            Err(err) => return TestResult::Fail(err.to_string()),
        };
        if parsed != reparsed {
            return TestResult::Fail(format!(
                "Reparsed does not match parsed:\n\tresult:   {reparsed}\n\texpected: {parsed}"
            ));
        }
        let checked = match parsed.check(&mut Default::default()) {
            Ok(ty) => ty,
            Err(err) => return TestResult::Fail(format!("Could not typecheck:\n{err}")),
        };
        let result = checked.to_string();
        let expected = test.config.ty;
        if result != expected {
            return TestResult::Fail(format!(
                    "Checked type does not match expected type:\n\tresult:  {result}\n\texpectd:{expected}"
                    ));
        }
        let evaled = match parsed.eval() {
            Ok(v) => v,
            Err(err) => return TestResult::Fail(format!("Could not evaluate term:\n{err}")),
        };
        let result = evaled.to_string();
        let expected = test.config.evaled;
        if result != expected {
            return TestResult::Fail(format!(
                "Evaluated does not match expected:\n\tresult:   {result}\n\texpected: {expected}"
            ));
        }

        TestResult::Success
    }

    fn load_tests(&self) -> Result<Vec<Test<Self::TestConf>>, Error> {
        load_dir(&self.source_dir, "stlc")
    }
}
