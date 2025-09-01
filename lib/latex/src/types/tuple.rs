use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::Tuple};

impl<Lang> LatexFmt for Tuple<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "({})",
            self.tys
                .iter()
                .map(|ty| ty.to_latex(conf))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
