use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{ListCase, Term};

impl<T> LatexFmt for ListCase<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{case}} {} \\text{{of}} \\{{ \\text{{Nil}} \\Rightarrow {} \\mid \\text{{Cons}}({},{}) \\Rightarrow {} \\}}",
            self.bound_term.to_latex(conf),
            self.nil_rhs.to_latex(conf),
            self.cons_fst,
            self.cons_rst,
            self.cons_rhs.to_latex(conf)
        )
    }
}
