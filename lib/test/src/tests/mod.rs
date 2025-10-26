use super::{config::TestConfig, test_result::TestResult};

pub mod check_test;
pub mod eval_test;
pub mod latex_buss_test;
pub mod latex_frac_test;
pub mod latex_grammar_test;
pub mod latex_trace_test;
pub mod parse_test;
pub mod reparse_test;

pub use check_test::CheckTest;
pub use eval_test::EvalTest;
pub use latex_buss_test::LatexTestBuss;
pub use latex_frac_test::LatexTestFrac;
pub use latex_grammar_test::LatexTestGrammar;
pub use latex_trace_test::LatexTestTrace;
pub use parse_test::ParseTest;
pub use reparse_test::ReparseTest;

/// Trait for a generic test
/// implemented by parse test, reparse test etc
/// the type
pub trait Test {
    /// The result type returned after running the test
    /// This is used so results can be reused between tests
    /// for example check tests require parsing results
    type Result;
    /// The type required to construct the test
    /// For example, the type check test requires a parsed program
    type Input;
    /// The name of the test
    /// displayed while running
    fn name(&self) -> String;
    /// The function to run the test
    fn run(&self) -> TestResult<Self::Result>;
    /// Create a test from a config
    /// if the config excludes the current test return `None`
    fn from_conf(conf: &TestConfig, input: Self::Input) -> Option<Self>
    where
        Self: Sized;

    /// Create a test and run it, then report
    /// Returns the result if the test was successful
    fn run_report(conf: &TestConfig, input: Self::Input) -> TestResult<Self::Result>
    where
        Self: Sized,
    {
        let slf = match Self::from_conf(conf, input) {
            None => return TestResult::Skipped,
            Some(tst) => tst,
        };
        let res = slf.run();
        res.report(&slf.name());
        res
    }
}
