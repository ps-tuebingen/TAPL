use super::super::LatexFmt;
use syntax::terms::{If, Term};

impl<T> LatexFmt for If<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "if ({}) \\{{ {} \\}} else \\{{ {} \\}}",
            self.if_cond, self.then_term, self.else_term
        )
    }
}
