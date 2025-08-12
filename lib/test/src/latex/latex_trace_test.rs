use crate::{paths::LATEX_OUT, test::Test, test_result::TestResult};
use latex::LatexFmt;
use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};
use syntax::{terms::Term, values::Value};
use trace::EvalTrace;

pub struct LatexTestTrace<'a, T, V>
where
    T: Term + LatexFmt,
    V: Value + LatexFmt,
{
    name: String,
    trace: &'a EvalTrace<T, V>,
}

impl<'a, T, V> LatexTestTrace<'a, T, V>
where
    T: Term + LatexFmt,
    V: Value + LatexFmt,
{
    pub fn new(name: &str, tr: &'a EvalTrace<T, V>) -> LatexTestTrace<'a, T, V> {
        LatexTestTrace {
            name: name.to_owned(),
            trace: tr,
        }
    }
}

impl<'a, T, V> Test<()> for LatexTestTrace<'a, T, V>
where
    T: Term + LatexFmt,
    V: Value + LatexFmt,
{
    fn name(&self) -> String {
        format!("Generating Latex for Evaluation Trace of {}", self.name)
    }

    fn run(&self) -> TestResult<()> {
        let latex_src = self.trace.to_document(&mut Default::default());

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
                    TestResult::Success(())
                } else {
                    TestResult::Fail("xelatex exited with non-zero exit status".to_owned())
                }
            }
        }
    }
}
