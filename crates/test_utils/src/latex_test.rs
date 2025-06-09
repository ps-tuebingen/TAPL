use super::{
    paths::LATEX_OUT,
    testsuite::{Test, TestResult},
};
use check::Typecheck;
use common::parse::Parse;
use derivation::latex::LatexFmt;
use std::{fmt, fs::File, io::Write, marker::PhantomData, path::PathBuf, process::Command};
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
        let latex_src = checked.to_document();
        let mut out_path = PathBuf::from(LATEX_OUT).join(&self.name);
        out_path.set_extension("tex");
        let mut out_file = match File::create(&out_path) {
            Ok(f) => f,
            Err(err) => return TestResult::from_err(err),
        };
        if let Err(err) = out_file.write_all(latex_src.as_bytes()) {
            return TestResult::from_err(err);
        };

        let mut latex_cmd = Command::new("pdflatex");
        latex_cmd.arg(out_path);
        latex_cmd.arg("-output-directory");
        latex_cmd.arg(LATEX_OUT);

        match latex_cmd.status() {
            Ok(_) => TestResult::Success,
            Err(err) => TestResult::from_err(err),
        }
    }
}
