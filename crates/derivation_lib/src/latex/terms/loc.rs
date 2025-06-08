use syntax::terms::{Loc,Term};
use super::super::LatexFmt;

impl<T> LatexFmt for Loc<T> where T:Term+LatexFmt{
    fn to_latex(&self)->String{
        format!("{}",self.loc)
    }
}
