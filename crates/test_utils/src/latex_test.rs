use super::{
    paths::LATEX_OUT,
    testsuite::{Test, TestResult},
};
use check::Typecheck;
use common::parse::Parse;
use derivation::latex::{LatexConfig, LatexFmt};
use std::{
    fmt,
    fs::File,
    io::Write,
    marker::PhantomData,
    path::PathBuf,
    process::{Command, Stdio},
};
use syntax::terms::Term;

pub struct LatexTest<T>
where
    T: Term + Parse + Typecheck<Term = T> + LatexFmt,
    T::Type: fmt::Display + LatexFmt,
{
    name: String,
    source: String,
    phantom: PhantomData<T>,
}

impl<T> LatexTest<T>
where
    T: Term + Parse + Typecheck<Term = T> + LatexFmt,
    T::Type: fmt::Display + LatexFmt,
{
    pub fn new(name: &str, source: &str) -> LatexTest<T> {
        LatexTest {
            name: name.to_owned(),
            source: source.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<T> Test for LatexTest<T>
where
    T: Term + Parse + Typecheck<Term = T> + LatexFmt,
    T::Type: fmt::Display + LatexFmt,
{
    fn name(&self) -> String {
        format!("Generating Latex for {}", self.name)
    }

    fn run(&self) -> TestResult {
        let parsed = match T::parse(self.source.clone()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let checked = match parsed.check_start() {
            Ok(c) => c,
            Err(err) => return TestResult::from_err(err),
        };
        let latex_src_bus = checked.to_document(&mut Default::default());
        let latex_src_frac = checked.to_document(&mut LatexConfig::new_frac());

        let mut out_path_bus = PathBuf::from(LATEX_OUT).join(format!("{}_bus", self.name));
        out_path_bus.set_extension("tex");

        let mut out_path_frac = PathBuf::from(LATEX_OUT).join(format!("{}_frac", &self.name));
        out_path_frac.set_extension("tex");

        let mut out_file_bus = match File::create(&out_path_bus) {
            Ok(f) => f,
            Err(err) => return TestResult::from_err(err),
        };
        let mut out_file_frac = match File::create(&out_path_frac) {
            Ok(f) => f,
            Err(err) => return TestResult::from_err(err),
        };

        if let Err(err) = out_file_bus.write_all(latex_src_bus.as_bytes()) {
            return TestResult::from_err(err);
        };
        if let Err(err) = out_file_frac.write_all(latex_src_frac.as_bytes()) {
            return TestResult::from_err(err);
        };

        let mut latex_cmd_bus = Command::new("xelatex");
        latex_cmd_bus.arg("-halt-on-error");
        latex_cmd_bus.arg(format!("-output-directory={LATEX_OUT}"));
        latex_cmd_bus.arg("-inteteraction=nonstopmode");
        latex_cmd_bus.arg(out_path_bus);
        latex_cmd_bus.stdout(Stdio::null());
        latex_cmd_bus.stderr(Stdio::null());

        let mut latex_cmd_frac = Command::new("xelatex");
        latex_cmd_frac.arg("-halt-on-error");
        latex_cmd_frac.arg(format!("-output-directory={LATEX_OUT}"));
        latex_cmd_frac.arg("-inteteraction=nonstopmode");
        latex_cmd_frac.arg(out_path_frac);
        latex_cmd_frac.stdout(Stdio::null());
        latex_cmd_frac.stderr(Stdio::null());

        match (latex_cmd_bus.status(), latex_cmd_frac.status()) {
            (Err(err), _) => TestResult::from_err(err),
            (_, Err(err)) => TestResult::from_err(err),
            (Ok(exit1), Ok(exit2)) => {
                if exit1.success() && exit2.success() {
                    TestResult::Success
                } else {
                    TestResult::Fail("xelatex exited with non-zero exit status".to_owned())
                }
            }
        }
    }
}
