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
use derivation::Derivation;
use language::Language;
use std::path::PathBuf;
use trace::EvalTrace;

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
    type Config: TestConfig;
    type Lang: Language;

    fn name(&self) -> &str;
    fn source_dir(&self) -> PathBuf;
    fn ext(&self) -> &str;

    fn configs(&self) -> Result<Vec<Self::Config>, Error> {
        load_dir(&self.source_dir(), self.ext())
    }

    fn run_parse(conf: &Self::Config) -> TestResult<<Self::Lang as Language>::Term> {
        let name = conf.name();
        let parse_test = ParseTest::<<Self::Lang as Language>::Term>::new(&name, &conf.contents());
        let parse_res = parse_test.run();
        parse_res.report(&parse_test.name());
        parse_res
    }

    fn run_reparse(name: &str, parsed: &<Self::Lang as Language>::Term) -> TestResult<()> {
        let reparse_test = ReparseTest::<<Self::Lang as Language>::Term>::new(name, &parsed);
        let res = reparse_test.run();
        res.report(&reparse_test.name());
        res
    }

    fn run_check(
        conf: &Self::Config,
        term: <Self::Lang as Language>::Term,
    ) -> TestResult<Derivation<<Self::Lang as Language>::Term, <Self::Lang as Language>::Type>>
    {
        let check_test =
            CheckTest::<<Self::Lang as Language>::Term>::new(&conf.name(), term, conf.ty());
        let res = check_test.run();
        res
    }

    fn run_eval(
        conf: &Self::Config,
        t: <Self::Lang as Language>::Term,
    ) -> TestResult<EvalTrace<<Self::Lang as Language>::Term, <Self::Lang as Language>::Value>>
    {
        let eval_test =
            EvalTest::<<Self::Lang as Language>::Term>::new(conf.name(), t, conf.evaluated());
        let res = eval_test.run();
        res.report(&eval_test.name());
        res
    }

    fn run_derivation_buss(
        name: &str,
        deriv: &Derivation<<Self::Lang as Language>::Term, <Self::Lang as Language>::Type>,
    ) -> TestResult<()> {
        let buss_test = LatexTestBuss::<
            <Self::Lang as Language>::Term,
            <Self::Lang as Language>::Type,
        >::new(name, deriv);
        let res = buss_test.run();
        res.report(&buss_test.name());
        res
    }

    fn run_derivation_frac(
        name: &str,
        deriv: &Derivation<<Self::Lang as Language>::Term, <Self::Lang as Language>::Type>,
    ) -> TestResult<()> {
        let frac_test = LatexTestFrac::<
            <Self::Lang as Language>::Term,
            <Self::Lang as Language>::Type,
        >::new(name, deriv);
        let res = frac_test.run();
        res.report(&frac_test.name());
        res
    }

    fn run_trace(
        name: &str,
        tr: &EvalTrace<<Self::Lang as Language>::Term, <Self::Lang as Language>::Value>,
    ) -> TestResult<()> {
        let trace_test = LatexTestTrace::<
            <Self::Lang as Language>::Term,
            <Self::Lang as Language>::Value,
        >::new(name, tr);
        let res = trace_test.run();
        res.report(&trace_test.name());
        res
    }

    fn run_conf(conf: &Self::Config) -> TestResult<()> {
        let name = conf.name();
        let inclusions = conf.inclusions();
        println!("Running tests for {}", name);

        let t = match Self::run_parse(conf) {
            TestResult::Success(t) => t,
            TestResult::Fail(err) => return TestResult::Fail(err),
        };

        if inclusions.reparse {
            Self::run_reparse(&name, &t);
        };

        if inclusions.check {
            let deriv = Self::run_check(&conf, t.clone());
            if let TestResult::Success(ref deriv) = deriv {
                if inclusions.derivation_buss {
                    Self::run_derivation_buss(&name, deriv);
                }
                if inclusions.derivation_frac {
                    Self::run_derivation_frac(&name, deriv);
                }
            }
        }

        if inclusions.eval {
            let trace = Self::run_eval(&conf, t);
            if let TestResult::Success(ref tr) = trace {
                if inclusions.trace {
                    Self::run_trace(&name, tr);
                }
            }
        }

        TestResult::Success(())
    }

    fn run_all(&self) -> Result<usize, Error> {
        println!("Running Test Suite {}\n", self.name());
        let configs = self.configs()?;
        let num_tests = configs.len();
        let mut num_fail = 0;
        for conf in configs {
            let result = Self::run_conf(&conf);
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
