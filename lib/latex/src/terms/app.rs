use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::App};

impl<Lang> LatexFmt for App<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "{}\\quad {}",
            self.fun.to_latex(conf),
            self.arg.to_latex(conf)
        )
    }
}
