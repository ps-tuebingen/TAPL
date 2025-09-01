use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::IsZero};

impl<Lang> LatexFmt for IsZero<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{iszero}}({})", self.term.to_latex(conf))
    }
}
