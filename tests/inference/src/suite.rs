use super::{BidirectionalTest, InferTest};
use std::path::PathBuf;
use test_common::{
    errors::Error,
    load_tests::{load_dir, TestContents},
    parse_test::ParseTest,
    reparse_test::ReparseTest,
    testsuite::{Test, TestSuite},
};

pub struct InferenceTests {
    source_path: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct InferenceConf {
    ty: String,
    can_bi: bool,
}

impl InferenceTests {
    pub fn new(path: PathBuf) -> InferenceTests {
        InferenceTests { source_path: path }
    }
}

impl TestSuite for InferenceTests {
    fn name(&self) -> String {
        "Type Inference".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<InferenceConf>> = load_dir(&self.source_path, "inf")?;
        let mut tests = vec![];
        for tst in contents {
            let parse_test =
                ParseTest::<inference::syntax::Term>::new(&tst.source_name, &tst.source_contents);
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test =
                ReparseTest::<inference::syntax::Term>::new(&tst.source_name, &tst.source_contents);
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let infer_test = InferTest::new(&tst.source_name, &tst.source_contents, &tst.conf.ty);
            tests.push(Box::new(infer_test) as Box<dyn Test>);
            if !tst.conf.can_bi {
                continue;
            }
            let bidirectional_test =
                BidirectionalTest::new(&tst.source_name, &tst.source_contents, &tst.conf.ty);
            tests.push(Box::new(bidirectional_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
