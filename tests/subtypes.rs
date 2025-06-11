use language::languages::subtypes::terms::Term;
use std::path::PathBuf;
use test_utils::{
    check_test::CheckTest,
    errors::Error,
    latex_buss_test::LatexTestBuss,
    latex_frac_test::LatexTestFrac,
    load_tests::{load_dir, TestContents},
    parse_test::ParseTest,
    paths::{EXAMPLES_PATH, SUBTYPES_PATH},
    reparse_test::ReparseTest,
    setup,
    testsuite::{Test, TestSuite},
};

pub struct SubtypesTests {
    source_path: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct SubtypesConf {
    ty: String,
}

impl SubtypesTests {
    pub fn new(path: PathBuf) -> SubtypesTests {
        SubtypesTests { source_path: path }
    }
}

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = SubtypesTests::new(examples_dir.join(SUBTYPES_PATH)).run_all()?;
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

impl TestSuite for SubtypesTests {
    fn name(&self) -> String {
        "Subtypes".to_owned()
    }

    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error> {
        let contents: Vec<TestContents<SubtypesConf>> = load_dir(&self.source_path, "sub")?;
        let mut tests = vec![];
        for tst in contents {
            let parse_test = ParseTest::<Term>::new(&tst.source_name, &tst.source_contents);
            tests.push(Box::new(parse_test) as Box<dyn Test>);
            let reparse_test = ReparseTest::<Term>::new(&tst.source_name, &tst.source_contents);
            tests.push(Box::new(reparse_test) as Box<dyn Test>);
            let check_test =
                CheckTest::<Term>::new(&tst.source_name, &tst.source_contents, &tst.conf.ty);
            tests.push(Box::new(check_test) as Box<dyn Test>);
            let latex_test = LatexTestBuss::<Term>::new(&tst.source_name, &tst.source_contents);
            tests.push(Box::new(latex_test) as Box<dyn Test>);
            let latex_test = LatexTestFrac::<Term>::new(&tst.source_name, &tst.source_contents);
            tests.push(Box::new(latex_test) as Box<dyn Test>);
        }
        Ok(tests)
    }
}
