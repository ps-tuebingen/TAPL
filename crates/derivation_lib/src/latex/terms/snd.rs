use super::super::LatexFmt;
use syntax::terms::{Term,Snd};

impl<T> LatexFmt for Snd<T> where T:Term+LatexFmt{
    fn to_latex(&self) -> String{
        format!("({}).1",self.term.to_latex())
    }
}
