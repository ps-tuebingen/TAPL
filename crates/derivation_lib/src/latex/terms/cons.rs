use super::super::LatexFmt;
use syntax::{
    terms::{Cons, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Cons<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "Cons[{}]({},{})",
            self.ty.to_latex(),
            self.head.to_latex(),
            self.tail.to_latex()
        )
    }
}
