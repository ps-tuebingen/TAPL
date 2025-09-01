use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::OpApp};

impl<Lang> LatexFmt for OpApp<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("({}[{}])", self.fun.to_latex(conf), self.arg.to_latex(conf))
    }
}
