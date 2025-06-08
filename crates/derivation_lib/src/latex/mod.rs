use crate::{Conclusion, Derivation};
use syntax::{terms::Term, types::Type};

mod env;
mod kind;
mod terms;
mod types;
mod untyped;
mod values;

pub trait LatexFmt {
    fn to_latex(&self) -> String;
}

impl<T, Ty> LatexFmt for Derivation<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        let mut out = "".to_owned();
        let conc_str = match self.premises.len() {
            0 => format!("\\UnaryInfC{{{}}}", self.conc.to_latex()),
            1 => format!("\\UnaryInfC{{{}}}", self.conc.to_latex()),
            2 => format!("\\BinaryInfC{{{}}}", self.conc.to_latex()),
            3 => format!("\\TryInfC{{{}}}", self.conc.to_latex()),
            4 => format!("\\QuaternaryInfC{{{}}}", self.conc.to_latex()),
            5 => format!("\\QuinaryInfC{{{}}}", self.conc.to_latex()),
            _ => panic!("Derivations with more than 5 premises are not supported"),
        };

        out += "\\begin{prooftree}\n";
        for prem in self.premises.iter() {
            out += "\t";
            out += &prem.to_latex();
            out += "\n";
        }
        out += "\\RightLabel{";
        out += &self.label.to_string();
        out += "}\n";
        out += &conc_str;
        out += "\n\\end{prooftree}";

        out
    }
}

impl<T, Ty> LatexFmt for Conclusion<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        let mut out = "".to_owned();

        out += &self.env.to_latex();
        out += " \\vdash ";
        out += &self.term.to_latex();
        out += " : ";
        out += &self.ty.to_latex();

        out
    }
}
