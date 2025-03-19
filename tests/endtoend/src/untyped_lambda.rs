use super::{
    errors::Error,
    test::{load_dir, TestContents},
    testsuite::{Test, TestResult, TestSuite},
};
use std::path::PathBuf;
use untyped_lambda::{
    eval::{eval, EvalOrder},
    parse::parse,
};

pub struct UntypedLambdaTests {
    source_dir: PathBuf,
}

impl UntypedLambdaTests {
    pub fn new(source_dir: PathBuf) -> UntypedLambdaTests {
        UntypedLambdaTests { source_dir }
    }
}

#[derive(serde::Deserialize)]
pub struct UntypedLambdaConf {
    evaluated: String,
}

pub struct ParseTest {
    source_name: String,
    source_contents: String,
}

impl Test for ParseTest {
    fn name(&self) -> String {
        format!("Parse {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        let mut src = self.source_contents.clone();
        match parse(&mut src) {
            Ok(_) => TestResult::Success,
            Err(err) => return TestResult::from_err(err),
        }
    }
}

pub struct ReparseTest {
    source_name: String,
    source_contents: String,
}

impl Test for ReparseTest {
    fn name(&self) -> String {
        format!("Reparse {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        let mut src = self.source_contents.clone();
        let parsed = match parse(&mut src) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let reparsed = match parse(&mut parsed.to_string()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        TestResult::from_eq(&reparsed, &parsed)
    }
}

pub struct EvalTest {
    source_name: String,
    source_contents: String,
    expected: String,
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
        let evaled = eval(parsed, EvalOrder::CBV);
        TestResult::from_eq(&evaled, &self.expected)
    }
}

impl TestSuite for UntypedLambdaTests {
    fn name(&self) -> String {
        "Untyped Lambda".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<UntypedLambdaConf>> = load_dir(&self.source_dir, "lam")?;
        let mut tests = vec![];
        for content in contents {
            let parse_test = ParseTest {
                source_name: content.source_name.clone(),
                source_contents: content.source_contents.clone(),
            };
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test = ReparseTest {
                source_name: content.source_name.clone(),
                source_contents: content.source_contents.clone(),
            };
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let eval_test = EvalTest {
                source_name: content.source_name,
                source_contents: content.source_contents,
                expected: content.conf.evaluated,
            };
            tests.push(Box::new(eval_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
