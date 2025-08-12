use crate::{paths::LATEX_OUT, test::Test, test_result::TestResult};
use derivations::ProgramDerivation;
use latex::LatexFmt;
use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};
use syntax::{terms::Term, types::Type};

pub struct LatexTestBuss<'a, T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    name: String,
    deriv: &'a ProgramDerivation<T, Ty>,
}

impl<'a, T, Ty> LatexTestBuss<'a, T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    pub fn new(name: &str, deriv: &'a ProgramDerivation<T, Ty>) -> LatexTestBuss<'a, T, Ty> {
        LatexTestBuss {
            name: name.to_owned(),
            deriv,
        }
    }
}

impl<'a, T, Ty> Test<()> for LatexTestBuss<'a, T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
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
}
