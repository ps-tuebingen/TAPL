use super::super::LatexFmt;
use syntax::terms::{variantcase::VariantPattern, Term, VariantCase};

impl<T> LatexFmt for VariantCase<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "case {} of \\{{ {} \\}}",
            self.bound_term.to_latex(),
            self.patterns
                .iter()
                .map(|pt| pt.to_latex())
                .collect::<Vec<String>>()
                .join(" \\mid ")
        )
    }
}

impl<T> LatexFmt for VariantPattern<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "\\langle {} = {} \\rangle \\Rightarrow {}",
            self.label,
            self.bound_var,
            self.rhs.to_latex()
        )
    }
}
