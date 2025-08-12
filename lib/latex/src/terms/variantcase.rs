use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Term, VariantCase, variantcase::VariantPattern};

impl<T> LatexFmt for VariantCase<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{case}} {} \\text{{of}} \\{{ {} \\}}",
            self.bound_term.to_latex(conf),
            self.patterns
                .iter()
                .map(|pt| pt.to_latex(conf))
                .collect::<Vec<String>>()
                .join(" \\mid ")
        )
    }
}

impl<T> LatexFmt for VariantPattern<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\langle {} = {} \\rangle \\Rightarrow {}",
            self.label.to_latex(conf),
            self.bound_var.to_latex(conf),
            self.rhs.to_latex(conf)
        )
    }
}
