use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::Sum};

impl<Lang> LatexFmt for Sum<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "({}+{})",
            self.left.to_latex(conf),
            self.right.to_latex(conf)
        )
    }
}
