use crate::{config::TestConfig, paths::LATEX_OUT, test_result::TestResult, tests::Test};
use derivations::ProgramDerivation;
use latex::{LatexConfig, LatexFmt};
use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};
use syntax::language::Language;

/// Test formatting a typing derivation using latex fractions+arrays
/// The Lang parameter is the language of the program ([`syntax::language::Language`])
pub struct LatexTestFrac<'a, Lang>
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

impl<'a, Lang> LatexTestFrac<'a, Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
    pub fn new(name: &str, deriv: &'a ProgramDerivation<Lang>) -> LatexTestFrac<'a, Lang> {
        LatexTestFrac {
            name: name.to_owned(),
            deriv,
        }
    }
}

impl<'a, Lang> Test for LatexTestFrac<'a, Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
    type Result = ();
    type Input = &'a ProgramDerivation<Lang>;

    fn name(&self) -> String {
        format!(
            "Generating Latex for Derivation Tree of {} (Frac + Array)",
            self.name
        )
    }

    fn run(&self) -> TestResult<()> {
        let latex_src = self.deriv.to_document(&mut LatexConfig::new_frac());

        let mut out_path = PathBuf::from(LATEX_OUT).join(format!("{}_frac", &self.name));
        out_path.set_extension("tex");

        let mut out_file = match File::create(&out_path) {
            Ok(f) => f,
            Err(err) => return TestResult::from_err(err),
        };

        if let Err(err) = out_file.write_all(latex_src.as_bytes()) {
            return TestResult::from_err(err);
        };

        let mut latex_cmd_frac = Command::new("xelatex");
        latex_cmd_frac.arg("-halt-on-error");
        latex_cmd_frac.arg(format!("-output-directory={LATEX_OUT}"));
        latex_cmd_frac.arg("-inteteraction=nonstopmode");
        latex_cmd_frac.arg(out_path);
        latex_cmd_frac.stdout(Stdio::null());
        latex_cmd_frac.stderr(Stdio::null());

        match latex_cmd_frac.status() {
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
        if !conf.include_frac() {
            None
        } else {
            Some(LatexTestFrac {
                name: conf.name.clone(),
                deriv,
            })
        }
    }
}
