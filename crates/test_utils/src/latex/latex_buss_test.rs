use crate::{
    paths::LATEX_OUT,
    test::{Test, TestConfig},
    test_result::TestResult,
};
use check::Typecheck;
use common::parse::Parse;
use latex::LatexFmt;
use std::{
    fmt,
    fs::File,
    io::Write,
    marker::PhantomData,
    path::PathBuf,
    process::{Command, Stdio},
};
use syntax::terms::Term;

pub struct LatexTestBuss<'a, T, Conf>
where
    T: Term + Parse + Typecheck<Term = T> + LatexFmt,
    T::Type: fmt::Display + LatexFmt,
    Conf: TestConfig,
{
    conf: &'a Conf,
    phantom: PhantomData<T>,
}

impl<'a, T, Conf> LatexTestBuss<'a, T, Conf>
where
    T: Term + Parse + Typecheck<Term = T> + LatexFmt,
    T::Type: fmt::Display + LatexFmt,
    Conf: TestConfig,
{
    pub fn new(conf: &'a Conf) -> LatexTestBuss<'a, T, Conf> {
        LatexTestBuss {
            conf,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, Conf> Test<'a> for LatexTestBuss<'a, T, Conf>
where
    T: Term + Parse + Typecheck<Term = T> + LatexFmt,
    T::Type: fmt::Display + LatexFmt,
    Conf: TestConfig,
{
    fn name(&self) -> String {
        format!(
            "Generating Latex for Derivation Trees of {} (Bussproofs)",
            self.conf.name()
        )
    }

    fn run(&self) -> TestResult {
        let parsed = match T::parse(self.conf.contents().to_owned()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let checked = match parsed.check_start() {
            Ok(c) => c,
            Err(err) => return TestResult::from_err(err),
        };
        let latex_src = checked.to_document(&mut Default::default());

        let mut out_path = PathBuf::from(LATEX_OUT).join(format!("{}_buss", self.conf.name()));
        out_path.set_extension("tex");

        let mut out_file = match File::create(&out_path) {
            Ok(f) => f,
            Err(err) => return TestResult::from_err(err),
        };

        if let Err(err) = out_file.write_all(latex_src.as_bytes()) {
            return TestResult::from_err(err);
        };

        let mut latex_cmd_bus = Command::new("xelatex");
        latex_cmd_bus.arg("-halt-on-error");
        latex_cmd_bus.arg(format!("-output-directory={LATEX_OUT}"));
        latex_cmd_bus.arg("-inteteraction=nonstopmode");
        latex_cmd_bus.arg(out_path);
        latex_cmd_bus.stdout(Stdio::null());
        latex_cmd_bus.stderr(Stdio::null());

        match latex_cmd_bus.status() {
            Err(err) => TestResult::from_err(err),
            Ok(exit) => {
                if exit.success() {
                    TestResult::Success
                } else {
                    TestResult::Fail("xelatex exited with non-zero exit status".to_owned())
                }
            }
        }
    }
}
