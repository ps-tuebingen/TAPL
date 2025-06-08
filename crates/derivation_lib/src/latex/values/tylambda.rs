use super::super::LatexFmt;
use eval::values::TyLambda;
use syntax::terms::Term;

impl<T> LatexFmt for TyLambda<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "\\lambda {}::{}.{}",
            self.var,
            self.annot.to_latex(),
            self.term.to_latex()
        )
    }
}
