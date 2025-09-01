use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::ListCase};

impl<Lang> LatexFmt for ListCase<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{case}} {} \\text{{of}} \\{{ \\text{{Nil}} \\Rightarrow {} \\mid \\text{{Cons}}({},{}) \\Rightarrow {} \\}}",
            self.bound_term.to_latex(conf),
            self.nil_rhs.to_latex(conf),
            self.cons_fst.to_latex(conf),
            self.cons_rst.to_latex(conf),
            self.cons_rhs.to_latex(conf)
        )
    }
}
