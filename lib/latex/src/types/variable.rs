use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::TypeVariable};

impl<Lang> LatexFmt for TypeVariable<Lang>
where
    Lang: Language,
{
    fn to_latex(&self, _conf: &mut LatexConfig) -> String {
        format!("\\text{{{}}}", self.v.to_latex(_conf))
    }
}
