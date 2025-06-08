use super::super::LatexFmt;
use syntax::terms::{Term, Tuple};

impl<T> LatexFmt for Tuple<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "\\{{ {} \\}}",
            self.terms
                .iter()
                .map(|t| t.to_latex())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
