use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, values::Pack};

impl<Lang> LatexFmt for Pack<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
    Lang::Value: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\{{*{},{}\\}} as {}",
            self.inner_ty.to_latex(conf),
            self.val.to_latex(conf),
            self.outer_ty.to_latex(conf)
        )
    }
}
