use crate::{
    paths::LATEX_OUT,
    test::{Test, TestConfig},
    test_result::TestResult,
};
use eval::Eval;
use latex::LatexFmt;
use parse::Parse;
use std::{
    fs::File,
    io::Write,
    marker::PhantomData,
    path::PathBuf,
    process::{Command, Stdio},
};
use syntax::terms::Term;

pub struct LatexTestTrace<'a, T, Conf>
where
    T: Term + Parse + Eval<Term = T> + LatexFmt,
    <T as Parse>::LeftRecArg: Default,
    T::Value: LatexFmt,
    Conf: TestConfig,
{
    conf: &'a Conf,
    phantom: PhantomData<T>,
}

impl<'a, T, Conf> LatexTestTrace<'a, T, Conf>
where
    T: Term + Parse + Eval<Term = T> + LatexFmt,
    <T as Parse>::LeftRecArg: Default,
    T::Value: LatexFmt,
    Conf: TestConfig,
{
    pub fn new(conf: &'a Conf) -> LatexTestTrace<'a, T, Conf> {
        LatexTestTrace {
            conf,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, Conf> Test<'a> for LatexTestTrace<'a, T, Conf>
where
    T: Term + Parse + Eval<Term = T> + LatexFmt,
    <T as Parse>::LeftRecArg: Default,
    T::Value: LatexFmt,
    Conf: TestConfig,
{
    fn name(&self) -> String {
        format!(
            "Generating Latex for Evaluation Trace of {}",
            self.conf.name()
        )
    }

    fn run(&self) -> TestResult {
        let parsed = match T::parse(self.conf.contents().to_owned()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let evaled = match parsed.eval_start() {
            Ok(v) => v,
            Err(err) => return TestResult::from_err(err),
        };
        let latex_src = evaled.to_document(&mut Default::default());

        let mut out_path = PathBuf::from(LATEX_OUT).join(format!("{}_trace", self.conf.name()));
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
