use super::super::LatexFmt;
use syntax::terms::{SomeCase, Term};

impl<T> LatexFmt for SomeCase<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "case {} of \\{{ Nothing \\Rightarrow {} \\mid Something({}) \\Rightarrow {} \\}}",
            self.bound_term.to_latex(),
            self.none_term.to_latex(),
            self.some_var,
            self.some_term.to_latex()
        )
    }
}
