use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Snd};

impl<Lang> LatexFmt for Snd<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("({}).\\text{{snd}}", self.term.to_latex(conf))
    }
}
