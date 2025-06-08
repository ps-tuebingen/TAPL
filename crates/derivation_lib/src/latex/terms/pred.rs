use super::super::LatexFmt;
use syntax::terms::{Pred,Term};

impl<T> LatexFmt for Pred<T> where T:Term+LatexFmt{
    fn to_latex(&self) -> String{
        format!("pred({})",self.term.to_latex())
    }
}
