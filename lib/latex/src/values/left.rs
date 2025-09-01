use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, values::Left};

impl<Lang> LatexFmt for Left<Lang>
where
    Lang: Language,
    Lang::Value: LatexFmt,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "inl({}) as {}",
            self.left_val.to_latex(conf),
            self.ty.to_latex(conf)
        )
    }
}
