use language::languages::typed_arithmetic::terms::Term;
use std::path::PathBuf;
use test_utils::{
    check_test::CheckTest,
    errors::Error,
    eval_test::EvalTest,
    latex_test::LatexTest,
    load_tests::{load_dir, TestContents},
    parse_test::ParseTest,
    paths::{EXAMPLES_PATH, TYPED_ARITH_PATH},
    reparse_test::ReparseTest,
    setup,
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

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = TypedArithTests::new(examples_dir.join(TYPED_ARITH_PATH)).run_all()?;
    let fail_str = if fails > 0 {
        format!("\x1b[31m{fails} fails\x1b[39m")
    } else {
        "0 fails".to_owned()
    };
    println!("Finished running tests with {fail_str}");
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
            let parse_test = ParseTest::<Term>::new(&content.source_name, &content.source_contents);
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test =
                ReparseTest::<Term>::new(&content.source_name, &content.source_contents);
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let eval_test = EvalTest::<Term>::new(
                &content.source_name,
                &content.source_contents,
                &content.conf.expected,
            );
            tests.push(Box::new(eval_test) as Box<dyn Test>);
            let check_test = CheckTest::<Term>::new(
                &content.source_name,
                &content.source_contents,
                &content.conf.ty,
            );
            tests.push(Box::new(check_test) as Box<dyn Test>);
            let latex_test = LatexTest::<Term>::new(&content.source_name, &content.source_contents);
            tests.push(Box::new(latex_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
