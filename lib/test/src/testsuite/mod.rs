use super::{
    config::TestConfig,
    setup,
    test_result::TestResult,
    tests::{
        CheckTest, EvalTest, LatexTestBuss, LatexTestFrac, LatexTestGrammar, LatexTestTrace,
        ParseTest, ReparseTest, Test,
    },
};
use check::Typecheck;
use clap::Parser;
use errors::test_error::TestError;
use eval::Eval;
use grammar::LanguageDescribe;
use latex::LatexFmt;
use parser::GroupParse;
use std::path::PathBuf;
use syntax::{language::Language, program::Program};

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

/// Abstraction over tests for each different language
/// This trait is implemented for each implementor of [`syntax::language::Language`]
pub trait TestSuite: Language {
    /// The base directory for all tests, usually `examples/lang`
    fn source_dir(&self) -> PathBuf;
    /// The file extension used for the language
    fn ext(&self) -> &str;

    fn run_check_tests(conf: &TestConfig, parse_res: &Program<Self>) -> usize
    where
        <Self as Language>::Term: Typecheck<Lang = Self> + LatexFmt,
        <Self as Language>::Type: LatexFmt,
    {
        let mut num_fails = 0;
        let check_res = match CheckTest::<Self>::run_report(&conf, parse_res.clone()) {
            TestResult::Success(res) => res,
            _ => return 3,
        };
        if matches!(
            LatexTestBuss::<Self>::run_report(&conf, &check_res),
            TestResult::Fail(_)
        ) {
            num_fails += 1;
        };
        if matches!(
            LatexTestFrac::<Self>::run_report(&conf, &check_res),
            TestResult::Fail(_)
        ) {
            num_fails += 1;
        }
        num_fails
    }

    fn run_eval_tests(conf: &TestConfig, parse_res: &Program<Self>) -> usize
    where
        <Self as Language>::Term: Eval<Lang = Self> + LatexFmt,
        <Self as Language>::Value: LatexFmt,
    {
        let mut num_fails = 0;
        let eval_res = match EvalTest::<Self>::run_report(&conf, parse_res.clone()) {
            TestResult::Success(res) => res,
            _ => return 2,
        };
        if matches!(
            LatexTestTrace::<Self>::run_report(&conf, &eval_res),
            TestResult::Fail(_)
        ) {
            num_fails += 1
        };
        num_fails
    }

    fn run_conf(conf: &TestConfig) -> usize
    where
        <Self as Language>::Term:
            GroupParse + Typecheck<Lang = Self> + Eval<Lang = Self> + LatexFmt,
        <Self as Language>::Type: GroupParse + LatexFmt,
        <Self as Language>::Value: LatexFmt,
        Self: LanguageDescribe,
    {
        let name = conf.name.clone();
        let mut num_fails = 0;
        println!("Running tests for {name}",);

        let parse_res = match ParseTest::<Self>::run_report(&conf, ()) {
            TestResult::Success(res) => res,
            _ => return num_fails + (conf.num_tests() - 1),
        };
        if matches!(
            ReparseTest::<Self>::run_report(&conf, parse_res.clone()),
            TestResult::Fail(_)
        ) {
            num_fails += 1
        };
        num_fails += Self::run_check_tests(&conf, &parse_res);
        num_fails += Self::run_eval_tests(&conf, &parse_res);
        num_fails
    }

    fn run_all(&self, args: &Args) -> Result<usize, TestError>
    where
        <Self as Language>::Term:
            GroupParse + Typecheck<Lang = Self> + Eval<Lang = Self> + LatexFmt,
        <Self as Language>::Type: GroupParse + LatexFmt,
        <Self as Language>::Value: LatexFmt,
        Self: LanguageDescribe,
    {
        println!("Running Test Suite {}", self.describe());

        let mut num_fails = 0;
        print!("\t");

        if matches!(
            LatexTestGrammar::<Self>::run_report(&TestConfig::empty(self.describe()), ()),
            TestResult::Fail(_)
        ) {
            num_fails += 1
        };

        println!();

        let configs = TestConfig::load_suite(&self.source_dir(), self.ext())?;
        let num_tests = configs.len() * configs.iter().fold(0, |num, conf| num + conf.num_tests());
        for mut conf in configs {
            args.update_conf(&mut conf);
            conf.update_features(&Self::features());
            println!(
                "Running tests for language {} with exclusions {:?}",
                self.describe(),
                conf.exclusions
            );

            let result = Self::run_conf(&conf);
            num_fails += result;
        }
        println!();
        println!(
            "\tRan {} tests: {} success, {} fail\n",
            num_tests,
            num_tests - num_fails,
            num_fails
        );
        Ok(num_fails)
    }

    fn run_report(&self) -> Result<(), TestError>
    where
        <Self as Language>::Term:
            GroupParse + Typecheck<Lang = Self> + Eval<Lang = Self> + LatexFmt,
        <Self as Language>::Type: GroupParse + LatexFmt,
        <Self as Language>::Value: LatexFmt,
        Self: LanguageDescribe,
    {
        setup()?;
        let args = Args::parse();

        let fails = self.run_all(&args)?;
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
