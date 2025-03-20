use super::{
    errors::Error,
    load_tests::{load_dir, TestContents},
    testsuite::{Test, TestResult},
    TestSuite,
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

pub struct ParseTest {
    source_name: String,
    source_contents: String,
}

impl Test for ParseTest {
    fn name(&self) -> String {
        format!("Parsing {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        match parse(self.source_contents.clone()) {
            Ok(_) => TestResult::Success,
            Err(err) => TestResult::from_err(err),
        }
    }
}

pub struct ReparseTest {
    source_name: String,
    source_contents: String,
}

impl Test for ReparseTest {
    fn name(&self) -> String {
        format!("Reparsing {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        let parsed = match parse(self.source_contents.clone()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        println!("parsed {parsed:?}");
        let reparsed = match parse(parsed.to_string()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        println!("reparsed {reparsed:?}");
        TestResult::from_eq(&reparsed, &parsed)
    }
}

pub struct TypecheckTest {
    source_name: String,
    source_contents: String,
    expected: String,
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
        println!("parsed {parsed:?}");
        let checked = match parsed.check(&mut Default::default()) {
            Ok(ty) => ty,
            Err(err) => return TestResult::from_err(err),
        };
        TestResult::from_eq(&checked, &self.expected)
    }
}

pub struct EvalTest {
    source_name: String,
    source_contents: String,
    expected: String,
}

impl Test for EvalTest {
    fn name(&self) -> String {
        format!("Evaluating {}", self.source_name)
    }

    fn run(&self) -> TestResult {
        let parsed = match parse(self.source_contents.clone()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let evaled = match parsed.eval() {
            Ok(v) => v,
            Err(err) => return TestResult::from_err(err),
        };
        TestResult::from_eq(&evaled, &self.expected)
    }
}

impl TestSuite for StlcTests {
    fn name(&self) -> String {
        "Stlc".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<StlcConf>> = load_dir(&self.source_dir, "stlc")?;
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
            let check_test = TypecheckTest {
                source_name: content.source_name.clone(),
                source_contents: content.source_contents.clone(),
                expected: content.conf.ty,
            };
            tests.push(Box::new(check_test) as Box<dyn Test>);
            let eval_test = EvalTest {
                source_name: content.source_name,
                source_contents: content.source_contents,
                expected: content.conf.evaled,
            };
            tests.push(Box::new(eval_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
