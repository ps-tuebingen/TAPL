use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::Fun};

impl<Lang> LatexFmt for Fun<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "{}\\rightarrow {}",
            self.from.to_latex(conf),
            self.to.to_latex(conf)
        )
    }
}
