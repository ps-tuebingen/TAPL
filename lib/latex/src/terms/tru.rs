use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::True};

impl<Lang> LatexFmt for True<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        "\\text{ true }".to_owned()
    }
}
