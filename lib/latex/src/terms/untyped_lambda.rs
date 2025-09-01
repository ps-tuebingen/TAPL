use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::UntypedLambda};

impl<Lang> LatexFmt for UntypedLambda<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\lambda {}.{}",
            self.var.to_latex(conf),
            self.body.to_latex(conf)
        )
    }
}
