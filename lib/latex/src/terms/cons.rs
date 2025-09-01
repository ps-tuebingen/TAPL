use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Cons};

impl<Lang> LatexFmt for Cons<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{Cons}}[{}]({},{})",
            self.ty.to_latex(conf),
            self.head.to_latex(conf),
            self.tail.to_latex(conf)
        )
    }
}
