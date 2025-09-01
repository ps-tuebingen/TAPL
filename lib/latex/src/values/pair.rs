use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, values::Pair};

impl<Lang> LatexFmt for Pair<Lang>
where
    Lang: Language,
    Lang::Value: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\{{ {},{} \\}}",
            self.fst.to_latex(conf),
            self.snd.to_latex(conf)
        )
    }
}
