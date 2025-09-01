use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Variable};

impl<Lang> LatexFmt for Variable<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, _conf: &mut LatexConfig) -> String {
        format!("\\text{{ {} }}", self.var.to_latex(_conf))
    }
}
