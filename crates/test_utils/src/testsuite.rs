use super::{
    check_test::{CheckConfig, CheckTest},
    errors::Error,
    eval_test::{EvalConfig, EvalTest},
    latex::{
        latex_buss_test::LatexTestBuss, latex_frac_test::LatexTestFrac,
        latex_trace_test::LatexTestTrace, LatexTestConf,
    },
    load_tests::load_dir,
    parse_test::ParseTest,
    reparse_test::ReparseTest,
    test::Test,
    test_result::TestResult,
};
use language::Language;
use std::path::PathBuf;

pub trait TestSuite {
    type Config: for<'b> serde::Deserialize<'b> + CheckConfig + EvalConfig + LatexTestConf;
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
                ParseTest::<<Self::Lang as Language>::Term, Self::Config>::new(&content);
            tests.push(Box::new(parse_test) as Box<dyn Test>);

            let reparse_test =
                ReparseTest::<<Self::Lang as Language>::Term, Self::Config>::new(&content);
            tests.push(Box::new(reparse_test) as Box<dyn Test>);

            let check_test =
                CheckTest::<<Self::Lang as Language>::Term, Self::Config>::new(&content);
            tests.push(Box::new(check_test) as Box<dyn Test>);

            let eval_test = EvalTest::<<Self::Lang as Language>::Term, Self::Config>::new(&content);
            tests.push(Box::new(eval_test) as Box<dyn Test>);

            let latex_test =
                LatexTestBuss::<<Self::Lang as Language>::Term, Self::Config>::new(&content);
            tests.push(Box::new(latex_test) as Box<dyn Test>);

            let latex_test =
                LatexTestFrac::<<Self::Lang as Language>::Term, Self::Config>::new(&content);
            tests.push(Box::new(latex_test) as Box<dyn Test>);

            let latex_test =
                LatexTestTrace::<<Self::Lang as Language>::Term, Self::Config>::new(&content);
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
}
