use super::{
    paths::LATEX_OUT,
    testsuite::{Test, TestResult},
};
use common::parse::Parse;
use eval::Eval;
use latex::LatexFmt;
use std::{
    fs::File,
    io::Write,
    marker::PhantomData,
    path::PathBuf,
    process::{Command, Stdio},
};
use syntax::terms::Term;

pub struct LatexTestTrace<T>
where
    T: Term + Parse + Eval<Term = T> + LatexFmt,
    T::Value: LatexFmt,
{
    name: String,
    source: String,
    phantom: PhantomData<T>,
}

impl<T> LatexTestTrace<T>
where
    T: Term + Parse + Eval<Term = T> + LatexFmt,
    T::Value: LatexFmt,
{
    pub fn new(name: &str, source: &str) -> LatexTestTrace<T> {
        LatexTestTrace {
            name: name.to_owned(),
            source: source.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<T> Test for LatexTestTrace<T>
where
    T: Term + Parse + Eval<Term = T> + LatexFmt,
    T::Value: LatexFmt,
{
    fn name(&self) -> String {
        format!("Generating Latex for Evaluation Trace of {}", self.name)
    }

    fn run(&self) -> TestResult {
        let parsed = match T::parse(self.source.clone()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let evaled = match parsed.eval_start() {
            Ok(v) => v,
            Err(err) => return TestResult::from_err(err),
        };
        let latex_src = evaled.to_document(&mut Default::default());

        let mut out_path = PathBuf::from(LATEX_OUT).join(format!("{}_trace", self.name));
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
