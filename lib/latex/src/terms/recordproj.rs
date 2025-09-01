use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::RecordProj};

impl<Lang> LatexFmt for RecordProj<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "({}).{}",
            self.record.to_latex(conf),
            self.label.to_latex(conf)
        )
    }
}
