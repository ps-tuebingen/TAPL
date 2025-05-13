use common::errors::Error;
use e2e_common::{
    check_test::CheckTest,
    eval_test::EvalTest,
    load_tests::{load_dir, TestContents},
    parse_test::ParseTest,
    paths::{EXAMPLES_PATH, UNTYPED_ARITH_PATH},
    reparse_test::ReparseTest,
    setup,
    testsuite::{Test, TestSuite},
};
use std::path::PathBuf;

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

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = TypedArithTests::new(examples_dir.join(UNTYPED_ARITH_PATH)).run_all()?;

    println!(
        "Finished running tests with \x1b[31m{} fails\x1b[39m",
        fails
    );
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}

impl TestSuite for TypedArithTests {
    fn name(&self) -> String {
        "Untyped Arithmetic".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<TypedArithConf>> = load_dir(&self.source_dir, "arith")?;
        let mut tests = vec![];
        for content in contents {
            let parse_test = ParseTest::<languages::typed_arithmetic::terms::Term>::new(
                &content.source_name,
                &content.source_contents,
            );
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test = ReparseTest::<languages::typed_arithmetic::terms::Term>::new(
                &content.source_name,
                &content.source_contents,
            );
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let eval_test = EvalTest::<languages::typed_arithmetic::terms::Term>::new(
                &content.source_name,
                &content.source_contents,
                &content.conf.expected,
            );
            tests.push(Box::new(eval_test) as Box<dyn Test>);
            let check_test = CheckTest::<languages::typed_arithmetic::terms::Term>::new(
                &content.source_name,
                &content.source_contents,
                &content.conf.ty,
            );
            tests.push(Box::new(check_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
