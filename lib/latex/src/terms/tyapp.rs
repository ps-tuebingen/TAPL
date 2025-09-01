use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::TyApp};

impl<Lang> LatexFmt for TyApp<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "(({})[{}])",
            self.fun.to_latex(conf),
            self.arg.to_latex(conf)
        )
    }
}
