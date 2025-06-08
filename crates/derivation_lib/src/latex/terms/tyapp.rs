use super::super::LatexFmt;
use syntax::{
    terms::{Term, TyApp},
    types::Type,
};

impl<T, Ty> LatexFmt for TyApp<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("(({})[{}])", self.fun.to_latex(), self.arg.to_latex())
    }
}
