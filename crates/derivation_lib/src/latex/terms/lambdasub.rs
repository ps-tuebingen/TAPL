use syntax::terms::{Term,LambdaSub};
use super::super::LatexFmt;

impl<T,Ty> LatexFmt for LambdaSub<T,Ty>{
    fn to_latex(&self) -> String{
        format!("\\lambda {}<:{}.{}",self.var,self.sup_ty,self.body)
    }
}
