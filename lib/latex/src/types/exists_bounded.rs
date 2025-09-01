use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::ExistsBounded};

impl<Lang> LatexFmt for ExistsBounded<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\{{\\exists {} <: {},{}\\}}",
            self.var.to_latex(conf),
            self.sup_ty.to_latex(conf),
            self.ty.to_latex(conf)
        )
    }
}
