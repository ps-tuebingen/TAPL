use super::{
    check_test::CheckTest,
    errors::Error,
    eval_test::EvalTest,
    latex::{
        latex_buss_test::LatexTestBuss, latex_frac_test::LatexTestFrac,
        latex_grammar_test::LatexTestGrammar, latex_trace_test::LatexTestTrace,
    },
    load_tests::load_dir,
    parse_test::ParseTest,
    paths::LATEX_OUT,
    reparse_test::ReparseTest,
    setup,
    test::{Test, TestConfig, TestInclusions},
    test_result::TestResult,
};
use clap::Parser;
use derivation::ProgramDerivation;
use language::Language;
use std::path::PathBuf;
use syntax::program::Program;
use trace::EvalTrace;

mod cli;
use cli::Args;

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

type LangProg<L> = Program<<L as Language>::Term, <L as Language>::Type>;
type LangTrace<L> = EvalTrace<<L as Language>::Term, <L as Language>::Value>;
type LangDerivation<L> = ProgramDerivation<<L as Language>::Term, <L as Language>::Type>;

pub trait TestSuite {
    type Config: TestConfig;
    type Lang: Language;

    fn name(&self) -> &str;
    fn source_dir(&self) -> PathBuf;
    fn ext(&self) -> &str;

    fn configs(&self) -> Result<Vec<Self::Config>, Error> {
        load_dir(&self.source_dir(), self.ext())
    }

    fn run_parse(conf: &Self::Config) -> TestResult<LangProg<Self::Lang>> {
        let name = conf.name();
        let parse_test =
            ParseTest::<<Self::Lang as Language>::Term, <Self::Lang as Language>::Type>::new(
                name,
                conf.contents(),
            );
        let parse_res = parse_test.run();
        parse_res.report(&parse_test.name());
        parse_res
    }

    fn run_reparse(name: &str, parsed: &LangProg<Self::Lang>) -> TestResult<()> {
        let reparse_test = ReparseTest::<
            <Self::Lang as Language>::Term,
            <Self::Lang as Language>::Type,
        >::new(name, parsed);
        let res = reparse_test.run();
        res.report(&reparse_test.name());
        res
    }

    fn run_check(
        conf: &Self::Config,
        prog: LangProg<Self::Lang>,
    ) -> TestResult<LangDerivation<Self::Lang>> {
        let check_test =
            CheckTest::<<Self::Lang as Language>::Term, <Self::Lang as Language>::Type>::new(
                conf.name(),
                prog,
                conf.ty(),
            );
        let res = check_test.run();
        res.report(&check_test.name());
        res
    }

    fn run_eval(
        conf: &Self::Config,
        prog: LangProg<Self::Lang>,
    ) -> TestResult<LangTrace<Self::Lang>> {
        let eval_test =
            EvalTest::<<Self::Lang as Language>::Term, <Self::Lang as Language>::Type>::new(
                conf.name(),
                prog,
                conf.evaluated(),
            );
        let res = eval_test.run();
        res.report(&eval_test.name());
        res
    }

    fn run_derivation_buss(
        name: &str,
        deriv: &ProgramDerivation<<Self::Lang as Language>::Term, <Self::Lang as Language>::Type>,
    ) -> TestResult<()> {
        std::fs::create_dir_all(PathBuf::from(LATEX_OUT)).unwrap();
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
        deriv: &ProgramDerivation<<Self::Lang as Language>::Term, <Self::Lang as Language>::Type>,
    ) -> TestResult<()> {
        std::fs::create_dir_all(PathBuf::from(LATEX_OUT)).unwrap();
        let frac_test = LatexTestFrac::<
            <Self::Lang as Language>::Term,
            <Self::Lang as Language>::Type,
        >::new(name, deriv);
        let res = frac_test.run();
        res.report(&frac_test.name());
        res
    }

    fn run_grammar(name: &str) -> TestResult<()> {
        std::fs::create_dir_all(PathBuf::from(LATEX_OUT)).unwrap();
        let grammar_test = LatexTestGrammar::<Self::Lang>::new(name);
        let res = grammar_test.run();
        res.report(&grammar_test.name());
        res
    }

    fn run_trace(
        name: &str,
        tr: &EvalTrace<<Self::Lang as Language>::Term, <Self::Lang as Language>::Value>,
    ) -> TestResult<()> {
        std::fs::create_dir_all(PathBuf::from(LATEX_OUT)).unwrap();
        let trace_test = LatexTestTrace::<
            <Self::Lang as Language>::Term,
            <Self::Lang as Language>::Value,
        >::new(name, tr);
        let res = trace_test.run();
        res.report(&trace_test.name());
        res
    }

    fn run_conf(conf: &Self::Config, inclusions: &TestInclusions) -> usize {
        let name = conf.name();
        let mut num_fails = 0;
        println!("Running tests for {}", name);

        print!("\t");
        let prog = match Self::run_parse(conf) {
            TestResult::Success(p) => p,
            TestResult::Fail(_) => return inclusions.num_tests(),
        };

        if inclusions.reparse {
            print!("\t");
            if matches!(Self::run_reparse(name, &prog), TestResult::Fail(_)) {
                num_fails += 1
            };
        };

        if inclusions.check {
            print!("\t");
            let deriv = Self::run_check(conf, prog.clone());
            match deriv {
                TestResult::Success(ref deriv) => {
                    if inclusions.derivation_buss {
                        print!("\t");
                        if matches!(Self::run_derivation_buss(name, deriv), TestResult::Fail(_)) {
                            num_fails += 1;
                        }
                    }
                    if inclusions.derivation_frac {
                        print!("\t");
                        if matches!(Self::run_derivation_frac(name, deriv), TestResult::Fail(_)) {
                            num_fails += 1;
                        }
                    }
                }
                TestResult::Fail(_) => {
                    num_fails += 1;
                    if inclusions.derivation_buss {
                        num_fails += 1;
                    }
                    if inclusions.derivation_frac {
                        num_fails += 1;
                    }
                }
            };
        }

        if inclusions.eval {
            print!("\t");
            let trace = Self::run_eval(conf, prog);
            match trace {
                TestResult::Success(ref tr) => {
                    if inclusions.trace {
                        print!("\t");
                        Self::run_trace(name, tr);
                    }
                }
                TestResult::Fail(_) => {
                    num_fails += 1;
                    if inclusions.trace {
                        num_fails += 1
                    }
                }
            }
        }

        if inclusions.grammar {
            print!("\t");
            let res = Self::run_grammar(&name);
            if matches!(res, TestResult::Fail(_)) {
                num_fails += 1;
            }
        }

        num_fails
    }

    fn run_all(&self, inclusions: &TestInclusions) -> Result<usize, Error> {
        println!("Running Test Suite {}\n", self.name());
        let configs = self.configs()?;
        let num_tests = configs.len() * inclusions.num_tests();
        let mut num_fail = 0;
        for conf in configs {
            let result = Self::run_conf(&conf, inclusions);
            num_fail += result;
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

        let inclusions = Args::parse().to_inclusions();

        let fails = self.run_all(&inclusions)?;
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
