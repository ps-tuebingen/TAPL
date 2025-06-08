use super::super::LatexFmt;
use syntax::terms::{Let, Term};

impl<T> LatexFmt for Let<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "let ({} = {}) in {}",
            self.var,
            self.bound_term.to_latex(),
            self.in_term.to_latex()
        )
    }
}
