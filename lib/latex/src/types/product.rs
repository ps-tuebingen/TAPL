use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::Product};

impl<Lang> LatexFmt for Product<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\{{ {} \\times {} \\}}",
            self.fst.to_latex(conf),
            self.snd.to_latex(conf)
        )
    }
}
