use crate::{config::TestConfig, paths::LATEX_OUT, test_result::TestResult, tests::Test};
use latex::LatexFmt;
use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};
use syntax::language::Language;
use trace::EvalTrace;

/// Tests formatting and compiling an evaluation trace for a given program
/// Lang is the Language ([`syntax::language::Language`]) the program is in
pub struct LatexTestTrace<'a, Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Value: LatexFmt,
{
    /// The name of the test
    name: String,
    /// The trace to format
    trace: &'a EvalTrace<Lang>,
}

impl<'a, Lang> LatexTestTrace<'a, Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Value: LatexFmt,
{
    /// Create a new trace test with given name and trace
    pub fn new(name: &str, tr: &'a EvalTrace<Lang>) -> LatexTestTrace<'a, Lang> {
        LatexTestTrace {
            name: name.to_owned(),
            trace: tr,
        }
    }
}

impl<'a, Lang> Test for LatexTestTrace<'a, Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Value: LatexFmt,
{
    type Result = ();
    type Input = &'a EvalTrace<Lang>;

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

    fn from_conf(conf: &TestConfig, trace: Self::Input) -> Option<Self> {
        if !conf.include_trace() {
            None
        } else {
            Some(LatexTestTrace {
                name: conf.name.clone(),
                trace,
            })
        }
    }
}
