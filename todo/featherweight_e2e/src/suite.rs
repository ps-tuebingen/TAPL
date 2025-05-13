use std::path::PathBuf;
use test_common::{
    check_test::CheckTest,
    errors::Error,
    load_tests::{load_dir, TestContents},
    parse_test::ParseTest,
    reparse_test::ReparseTest,
    testsuite::{Test, TestSuite},
};

pub struct FeatherweightTests {
    source_path: PathBuf,
}

#[derive(serde::Deserialize)]
struct FeatherweightConf {}

impl FeatherweightTests {
    pub fn new(path: PathBuf) -> FeatherweightTests {
        FeatherweightTests { source_path: path }
    }
}

impl TestSuite for FeatherweightTests {
    fn name(&self) -> String {
        "Featherweight Java".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<FeatherweightConf>> = load_dir(&self.source_path, "java")?;
        let mut tests = vec![];
        for tst in contents {
            let parse_test = ParseTest::<featherweight::syntax::ClassTable>::new(
                &tst.source_name,
                &tst.source_contents,
            );
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test = ReparseTest::<featherweight::syntax::ClassTable>::new(
                &tst.source_name,
                &tst.source_contents,
            );
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let check_test = CheckTest::<featherweight::syntax::ClassTable>::new(
                &tst.source_name,
                &tst.source_contents,
                "OK",
            );
            tests.push(Box::new(check_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
