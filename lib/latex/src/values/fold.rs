use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, values::Fold};

impl<Lang> LatexFmt for Fold<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
    Lang::Value: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "fold[{}]({})",
            self.ty.to_latex(conf),
            self.val.to_latex(conf)
        )
    }
}
