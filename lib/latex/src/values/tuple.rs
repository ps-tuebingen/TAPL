use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, values::Tuple};

impl<Lang> LatexFmt for Tuple<Lang>
where
    Lang: Language,
    Lang::Value: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "( {} )",
            self.vals
                .iter()
                .map(|v| v.to_latex(conf))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
