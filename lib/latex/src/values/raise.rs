use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, values::Raise};

impl<Lang> LatexFmt for Raise<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
    Lang::Value: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "raise[{}]({} : {})",
            self.cont_ty.to_latex(conf),
            self.val.to_latex(conf),
            self.cont_ty.to_latex(conf)
        )
    }
}
