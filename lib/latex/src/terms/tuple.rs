use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Tuple};

impl<Lang> LatexFmt for Tuple<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "( {} )",
            self.terms
                .iter()
                .map(|t| t.to_latex(conf))
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
