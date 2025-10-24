use super::{
    check_test::CheckTest,
    config::TestConfig,
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
    test::{Test, TestInclusions},
    test_result::TestResult,
};
use check::Typecheck;
use clap::Parser;
use derivations::ProgramDerivation;
use errors::test_error::TestError;
use eval::Eval;
use grammar::LanguageDescribe;
use latex::LatexFmt;
use parser::GroupParse;
use std::path::PathBuf;
use syntax::{language::Language, program::Program};
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

pub trait TestSuite {
    type Lang: Language;

    fn name(&self) -> &str;
    fn source_dir(&self) -> PathBuf;
    fn ext(&self) -> &str;

    fn configs(&self) -> Result<Vec<TestConfig>, TestError> {
        load_dir(&self.source_dir(), self.ext())
    }

    fn run_parse(conf: &TestConfig) -> TestResult<Program<Self::Lang>>
    where
        <Self::Lang as Language>::Term: GroupParse,
        <Self::Lang as Language>::Type: GroupParse,
    {
        let name = conf.name.clone();
        let parse_test = ParseTest::<Self::Lang>::new(&name, &conf.contents);
        let parse_res = parse_test.run();
        parse_res.report(&parse_test.name());
        parse_res
    }

    fn run_reparse(name: &str, parsed: &Program<Self::Lang>) -> TestResult<()>
    where
        <Self::Lang as Language>::Term: GroupParse,
        <Self::Lang as Language>::Type: GroupParse,
    {
        let reparse_test = ReparseTest::<Self::Lang>::new(name, parsed);
        let res = reparse_test.run();
        res.report(&reparse_test.name());
        res
    }

    fn run_check(
        conf: &TestConfig,
        prog: Program<Self::Lang>,
    ) -> TestResult<ProgramDerivation<Self::Lang>>
    where
        <Self::Lang as Language>::Term: Typecheck<Lang = Self::Lang>,
    {
        let check_test = CheckTest::<Self::Lang>::new(&conf.name, prog, &conf.ty);
        let res = check_test.run();
        res.report(&check_test.name());
        res
    }

    fn run_eval(conf: &TestConfig, prog: Program<Self::Lang>) -> TestResult<EvalTrace<Self::Lang>>
    where
        <Self::Lang as Language>::Term: Eval<Lang = Self::Lang>,
    {
        let eval_test = EvalTest::<Self::Lang>::new(&conf.name, prog, &conf.evaluated);
        let res = eval_test.run();
        res.report(&eval_test.name());
        res
    }

    fn run_derivation_buss(name: &str, deriv: &ProgramDerivation<Self::Lang>) -> TestResult<()>
    where
        <Self::Lang as Language>::Term: LatexFmt,
        <Self::Lang as Language>::Type: LatexFmt,
    {
        std::fs::create_dir_all(PathBuf::from(LATEX_OUT)).unwrap();
        let buss_test = LatexTestBuss::<Self::Lang>::new(name, deriv);
        let res = buss_test.run();
        res.report(&buss_test.name());
        res
    }

    fn run_derivation_frac(name: &str, deriv: &ProgramDerivation<Self::Lang>) -> TestResult<()>
    where
        <Self::Lang as Language>::Term: LatexFmt,
        <Self::Lang as Language>::Type: LatexFmt,
    {
        std::fs::create_dir_all(PathBuf::from(LATEX_OUT)).unwrap();
        let frac_test = LatexTestFrac::<Self::Lang>::new(name, deriv);
        let res = frac_test.run();
        res.report(&frac_test.name());
        res
    }

    fn run_grammar(name: &str) -> TestResult<()>
    where
        Self::Lang: LanguageDescribe,
    {
        std::fs::create_dir_all(PathBuf::from(LATEX_OUT)).unwrap();
        let grammar_test = LatexTestGrammar::<Self::Lang>::new(name);
        let res = grammar_test.run();
        res.report(&grammar_test.name());
        res
    }

    fn run_trace(name: &str, tr: &EvalTrace<Self::Lang>) -> TestResult<()>
    where
        <Self::Lang as Language>::Term: LatexFmt,
        <Self::Lang as Language>::Type: LatexFmt,
        <Self::Lang as Language>::Value: LatexFmt,
    {
        std::fs::create_dir_all(PathBuf::from(LATEX_OUT)).unwrap();
        let trace_test = LatexTestTrace::<Self::Lang>::new(name, tr);
        let res = trace_test.run();
        res.report(&trace_test.name());
        res
    }

    fn run_conf(conf: &TestConfig, inclusions: &TestInclusions) -> usize
    where
        <Self::Lang as Language>::Term:
            GroupParse + Typecheck<Lang = Self::Lang> + Eval<Lang = Self::Lang> + LatexFmt,
        <Self::Lang as Language>::Type: GroupParse + LatexFmt,
        <Self::Lang as Language>::Value: LatexFmt,
    {
        let name = conf.name.clone();
        let mut num_fail = 0;
        println!("Running tests for {name}",);

        print!("\t");
        let prog = match Self::run_parse(conf) {
            TestResult::Success(p) => p,
            TestResult::Fail(_) => return inclusions.num_tests(),
        };

        if inclusions.reparse {
            print!("\t");
            if matches!(Self::run_reparse(&name, &prog), TestResult::Fail(_)) {
                num_fail += 1
            };
        };

        if inclusions.check {
            print!("\t");
            let deriv = Self::run_check(conf, prog.clone());
            match deriv {
                TestResult::Success(ref deriv) => {
                    if inclusions.derivation_buss {
                        print!("\t");
                        if matches!(Self::run_derivation_buss(&name, deriv), TestResult::Fail(_)) {
                            num_fail += 1;
                        }
                    }
                    if inclusions.derivation_frac {
                        print!("\t");
                        if matches!(Self::run_derivation_frac(&name, deriv), TestResult::Fail(_)) {
                            num_fail += 1;
                        }
                    }
                }
                TestResult::Fail(_) => {
                    num_fail += 1;
                    if inclusions.derivation_buss {
                        num_fail += 1;
                    }
                    if inclusions.derivation_frac {
                        num_fail += 1;
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
                        Self::run_trace(&name, tr);
                    }
                }
                TestResult::Fail(_) => {
                    num_fail += 1;
                    if inclusions.trace {
                        num_fail += 1
                    }
                }
            }
        }

        num_fail
    }

    fn run_all(&self, inclusions: &TestInclusions) -> Result<usize, TestError>
    where
        <Self::Lang as Language>::Term:
            GroupParse + Typecheck<Lang = Self::Lang> + Eval<Lang = Self::Lang> + LatexFmt,
        <Self::Lang as Language>::Type: GroupParse + LatexFmt,
        <Self::Lang as Language>::Value: LatexFmt,
        Self::Lang: LanguageDescribe,
    {
        println!("Running Test Suite {}", self.name());

        let mut num_fail = 0;
        if inclusions.grammar {
            print!("\t");
            let res = Self::run_grammar(self.name());
            if matches!(res, TestResult::Fail(_)) {
                num_fail += 1;
            }
        }
        println!();

        let configs = self.configs()?;
        let num_tests = configs.len() * inclusions.num_tests();
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

    fn run_report(&self) -> Result<(), TestError>
    where
        <Self::Lang as Language>::Term:
            GroupParse + Typecheck<Lang = Self::Lang> + Eval<Lang = Self::Lang> + LatexFmt,
        <Self::Lang as Language>::Type: GroupParse + LatexFmt,
        <Self::Lang as Language>::Value: LatexFmt,
        Self::Lang: LanguageDescribe,
    {
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

    fn run_untyped(&self) -> Result<(), TestError>
    where
        Self::Lang: LanguageDescribe,
        <Self::Lang as Language>::Term: GroupParse + Eval<Lang = Self::Lang> + LatexFmt,
        <Self::Lang as Language>::Type: GroupParse + LatexFmt,
        <Self::Lang as Language>::Value: LatexFmt,
    {
        setup()?;
        let mut inclusions = Args::parse().to_inclusions();
        inclusions.check = false;
        inclusions.derivation_buss = false;
        inclusions.derivation_frac = false;

        println!("Running Test Suite {}", self.name());

        let mut num_fail = 0;
        if inclusions.grammar {
            print!("\t");
            let res = Self::run_grammar(self.name());
            if matches!(res, TestResult::Fail(_)) {
                num_fail += 1;
            }
        }
        println!();

        let configs = self.configs()?;
        let num_tests = configs.len() * inclusions.num_tests();
        for conf in configs {
            let name = conf.name.clone();
            println!("Running tests for {name}",);

            print!("\t");
            let prog = match Self::run_parse(&conf) {
                TestResult::Success(p) => p,
                TestResult::Fail(_) => {
                    num_fail += 1;
                    continue;
                }
            };

            if inclusions.reparse {
                print!("\t");
                if matches!(Self::run_reparse(&name, &prog), TestResult::Fail(_)) {
                    num_fail += 1
                };
            };

            if inclusions.eval {
                print!("\t");
                let trace = Self::run_eval(&conf, prog);
                match trace {
                    TestResult::Success(ref tr) => {
                        if inclusions.trace {
                            print!("\t");
                            Self::run_trace(&name, tr);
                        }
                    }
                    TestResult::Fail(_) => {
                        num_fail += 1;
                        if inclusions.trace {
                            num_fail += 1
                        }
                    }
                }
            }
        }

        println!();
        println!(
            "\tRan {} tests: {} success, {} fail\n",
            num_tests,
            num_tests - num_fail,
            num_fail
        );
        Ok(())
    }
}
