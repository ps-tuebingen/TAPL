use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, values::Right};

impl<Lang> LatexFmt for Right<Lang>
where
    Lang: Language,
    Lang::Value: LatexFmt,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "inl({}) as {}",
            self.right_val.to_latex(conf),
            self.ty.to_latex(conf)
        )
    }
}
