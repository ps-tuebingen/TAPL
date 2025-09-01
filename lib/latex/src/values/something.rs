use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, values::Something};

impl<Lang> LatexFmt for Something<Lang>
where
    Lang: Language,
    Lang::Value: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("something({})", self.val.to_latex(conf))
    }
}
