use super::super::LatexFmt;
use syntax::terms::{ListCase, Term};

impl<T> LatexFmt for ListCase<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "case {} of \\{{ Nil \\Rightarrow {} \\mid Cons({},{}) \\Rightarrow {} \\}}",
            self.bound_term.to_latex(),
            self.nil_rhs.to_latex(),
            self.cons_fst,
            self.cons_rst,
            self.cons_rhs.to_latex()
        )
    }
}
