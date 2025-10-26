use crate::{config::TestConfig, paths::LATEX_OUT, test_result::TestResult, tests::Test};
use derivations::ProgramDerivation;
use latex::LatexFmt;
use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};
use syntax::language::Language;

/// Test formatting and compiling a typing derivation in latex
/// uses bussproofs for formatting
/// For some examples this has to be skipped because of limits on derivation sizes
/// The derivation is in the given languge ([`syntax::language::Language`])
pub struct LatexTestBuss<'a, Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
    /// The name of the program
    name: String,
    /// The derivation to format
    deriv: &'a ProgramDerivation<Lang>,
}

impl<'a, Lang> LatexTestBuss<'a, Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
    /// Create a new test with given name and derivation
    pub fn new(name: &str, deriv: &'a ProgramDerivation<Lang>) -> LatexTestBuss<'a, Lang> {
        LatexTestBuss {
            name: name.to_owned(),
            deriv,
        }
    }
}

impl<'a, Lang> Test for LatexTestBuss<'a, Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
    type Result = ();
    type Input = &'a ProgramDerivation<Lang>;

    fn name(&self) -> String {
        format!(
            "Generating Latex for Derivation Trees of {} (Bussproofs)",
            self.name
        )
    }

    fn run(&self) -> TestResult<()> {
        let latex_src = self.deriv.to_document(&mut Default::default());

        let mut out_path = PathBuf::from(LATEX_OUT).join(format!("{}_buss", self.name));
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

    fn from_conf(conf: &TestConfig, deriv: Self::Input) -> Option<Self> {
        if !conf.include_buss() {
            None
        } else {
            Some(LatexTestBuss {
                name: conf.name.clone(),
                deriv,
            })
        }
    }
}
