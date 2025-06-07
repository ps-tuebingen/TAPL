use syntax::kinds::Kind;
use super::LatexFmt;

impl LatexFmt for Kind{
    fn to_latex(&self) -> String{
        match self{
            Kind::Star => "*".to_owned(),
            Kind::Arrow(from,to) => format!("{from} \\Rightarrow {to}")
        }
    }
}
