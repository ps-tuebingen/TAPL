use super::{
    check_test::CheckTest,
    errors::Error,
    eval_test::EvalTest,
    latex::{
        latex_buss_test::LatexTestBuss, latex_frac_test::LatexTestFrac,
        latex_trace_test::LatexTestTrace,
    },
    load_tests::load_dir,
    parse_test::ParseTest,
    reparse_test::ReparseTest,
    setup,
    test::{Test, TestConfig},
    test_result::TestResult,
};
use language::Language;
use std::path::PathBuf;

pub mod bounded_quantification;
pub mod exceptions;
pub mod existential;
pub mod f_omega;
pub mod f_omega_sub;
pub mod lambda_omega;
pub mod recursive;
pub mod references;
pub mod stlc;
pub mod subtypes;
pub mod system_f;
pub mod typed_arithmetic;
pub mod untyped_arithmetic;
pub mod untyped_lambda;

pub trait TestSuite {
    type Config: for<'a> serde::Deserialize<'a> + TestConfig;
    type Lang: Language;

    fn name(&self) -> &str;
    fn source_dir(&self) -> PathBuf;
    fn ext(&self) -> &str;

    fn configs(&self) -> Result<Vec<Self::Config>, Error> {
        load_dir(&self.source_dir(), self.ext())
    }

    fn load<'a>(
        &'a self,
        contents: &'a [Self::Config],
    ) -> Result<Vec<Box<dyn Test<'a> + 'a>>, Error> {
        let mut tests = vec![];
        for content in contents {
            let parse_test =
                ParseTest::<<Self::Lang as Language>::Term, Self::Config>::new(content);
            tests.push(Box::new(parse_test) as Box<dyn Test>);

            let reparse_test =
                ReparseTest::<<Self::Lang as Language>::Term, Self::Config>::new(content);
            tests.push(Box::new(reparse_test) as Box<dyn Test>);

            let check_test =
                CheckTest::<<Self::Lang as Language>::Term, Self::Config>::new(content);
            tests.push(Box::new(check_test) as Box<dyn Test>);

            let eval_test = EvalTest::<<Self::Lang as Language>::Term, Self::Config>::new(content);
            tests.push(Box::new(eval_test) as Box<dyn Test>);

            let latex_test =
                LatexTestBuss::<<Self::Lang as Language>::Term, Self::Config>::new(content);
            tests.push(Box::new(latex_test) as Box<dyn Test>);

            let latex_test =
                LatexTestFrac::<<Self::Lang as Language>::Term, Self::Config>::new(content);
            tests.push(Box::new(latex_test) as Box<dyn Test>);

            let latex_test =
                LatexTestTrace::<<Self::Lang as Language>::Term, Self::Config>::new(content);
            tests.push(Box::new(latex_test) as Box<dyn Test>);
        }
        Ok(tests)
    }

    fn run_all(&self) -> Result<usize, Error> {
        println!("Running Test Suite {}\n", self.name());
        let configs = self.configs()?;
        let tests = self.load(&configs)?;
        let num_tests = tests.len();
        let mut num_fail = 0;
        for test in tests {
            let result = test.run();
            result.report(&test.name());
            if matches!(result, TestResult::Fail(_)) {
                num_fail += 1
            }
        }
        println!();
        println!(
            "\tRan {} tests: {} success, {} fail\n",
            num_tests,
            num_tests - num_fail,
            num_fail
        );
        Ok(num_fail)
    }

    fn run_report(&self) -> Result<(), Error> {
        setup()?;

        let fails = self.run_all()?;
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
}
