use super::{EvalCtxTest, EvalTest, ParseTest, ReparseTest, TypecheckTest};
use std::path::PathBuf;
use test_common::{
    errors::Error,
    load_tests::{load_dir, TestContents},
    testsuite::{Test, TestSuite},
};

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

impl TestSuite for StlcTests {
    fn name(&self) -> String {
        "Stlc".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<StlcConf>> = load_dir(&self.source_dir, "stlc")?;
        let mut tests = vec![];
        for content in contents {
            let parse_test = ParseTest::new(&content.source_name, &content.source_contents);
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test = ReparseTest::new(&content.source_name, &content.source_contents);
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let check_test = TypecheckTest::new(
                &content.source_name,
                &content.source_contents,
                &content.conf.ty,
            );
            tests.push(Box::new(check_test) as Box<dyn Test>);
            let eval_test = EvalTest::new(
                &content.source_name,
                &content.source_contents,
                &content.conf.evaled,
            );
            tests.push(Box::new(eval_test) as Box<dyn Test>);
            let eval_ctx_test = EvalCtxTest::new(
                &content.source_name,
                &content.source_contents,
                &content.conf.evaled,
            );
            tests.push(Box::new(eval_ctx_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
