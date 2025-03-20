use super::{
    errors::Error,
    load_tests::{load_dir, TestContents},
    testsuite::{Test, TestResult, TestSuite},
};
use std::path::PathBuf;
use untyped_arithmetic::parse::parse;

pub struct UntypedArithTests {
    source_dir: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct UntypedArithConf {
    expected: String,
}

impl UntypedArithTests {
    pub fn new(source_dir: PathBuf) -> UntypedArithTests {
        UntypedArithTests { source_dir }
    }
}

pub struct ParseTest {
    source_name: String,
    source: String,
}

impl Test for ParseTest {
    fn name(&self) -> String {
        format!("Parsing {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        match parse(self.source.clone()) {
            Ok(_) => TestResult::Success,
            Err(err) => TestResult::from_err(err),
        }
    }
}

pub struct ReparseTest {
    source_name: String,
    source: String,
}

impl Test for ReparseTest {
    fn name(&self) -> String {
        format!("Reparsing {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        let parsed = match parse(self.source.clone()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let reparsed = match parse(parsed.to_string()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        TestResult::from_eq(&reparsed, &parsed)
    }
}

pub struct EvalTest {
    source_name: String,
    source: String,
    expected: String,
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
        let evaled = parsed.eval();
        TestResult::from_eq(&evaled, &self.expected)
    }
}

impl TestSuite for UntypedArithTests {
    fn name(&self) -> String {
        "Untyped Arithmetic".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<UntypedArithConf>> = load_dir(&self.source_dir, "arith")?;
        let mut tests = vec![];
        for content in contents {
            let parse_test = ParseTest {
                source_name: content.source_name.clone(),
                source: content.source_contents.clone(),
            };
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test = ReparseTest {
                source_name: content.source_name.clone(),
                source: content.source_contents.clone(),
            };
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let eval_test = EvalTest {
                source_name: content.source_name,
                source: content.source_contents,
                expected: content.conf.expected,
            };
            tests.push(Box::new(eval_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
