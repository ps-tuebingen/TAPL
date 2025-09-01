use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Something};

impl<Lang> LatexFmt for Something<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{something}}({})", self.term.to_latex(conf))
    }
}
