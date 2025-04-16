use std::path::PathBuf;
use test_common::{
    check_test::CheckTest,
    errors::Error,
    eval_test::EvalTest,
    load_tests::{load_dir, TestContents},
    parse_test::ParseTest,
    reparse_test::ReparseTest,
    testsuite::{Test, TestSuite},
};

pub struct TypedArithTests {
    source_dir: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct TypedArithConf {
    ty: String,
    expected: String,
}

impl TypedArithTests {
    pub fn new(source_dir: PathBuf) -> TypedArithTests {
        TypedArithTests { source_dir }
    }
}

impl TestSuite for TypedArithTests {
    fn name(&self) -> String {
        "Untyped Arithmetic".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<TypedArithConf>> = load_dir(&self.source_dir, "arith")?;
        let mut tests = vec![];
        for content in contents {
            let parse_test = ParseTest::<typed_arithmetic::terms::Term>::new(
                &content.source_name,
                &content.source_contents,
            );
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test = ReparseTest::<typed_arithmetic::terms::Term>::new(
                &content.source_name,
                &content.source_contents,
            );
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let eval_test = EvalTest::<typed_arithmetic::terms::Term>::new(
                &content.source_name,
                &content.source_contents,
                &content.conf.expected,
            );
            tests.push(Box::new(eval_test) as Box<dyn Test>);
            let check_test = CheckTest::<typed_arithmetic::terms::Term>::new(
                &content.source_name,
                &content.source_contents,
                &content.conf.ty,
            );
            tests.push(Box::new(check_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
