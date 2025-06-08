use super::super::LatexFmt;
use syntax::terms::{SumCase, Term};

impl<T> LatexFmt for SumCase<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "case {} of \\{{ inl({}) \\Rightarrow {} \\mid inr({}) \\Rightarrow {} \\}}",
            self.bound_term.to_latex(),
            self.left_var,
            self.left_term.to_latex(),
            self.right_var,
            self.right_term.to_latex()
        )
    }
}
