use super::{EvalTest, ParseTest, ReparseTest};
use std::path::PathBuf;
use test_common::{
    errors::Error,
    load_tests::{load_dir, TestContents},
    testsuite::{Test, TestSuite},
};

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

impl TestSuite for UntypedArithTests {
    fn name(&self) -> String {
        "Untyped Arithmetic".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<UntypedArithConf>> = load_dir(&self.source_dir, "arith")?;
        let mut tests = vec![];
        for content in contents {
            let parse_test = ParseTest::new(&content.source_name, &content.source_contents);
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test = ReparseTest::new(&content.source_name, &content.source_contents);
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let eval_test = EvalTest::new(
                &content.source_name,
                &content.source_contents,
                &content.conf.expected,
            );
            tests.push(Box::new(eval_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
