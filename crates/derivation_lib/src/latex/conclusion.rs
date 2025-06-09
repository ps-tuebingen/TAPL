use super::{Conclusion, LatexConfig, LatexFmt};
use syntax::{terms::Term, types::Type};

impl<T, Ty> LatexFmt for Conclusion<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let mut out = "".to_owned();

        out += &self.env.to_latex(conf);
        out += " \\vdash ";
        out += &self.term.to_latex(conf);
        out += " : ";
        out += &self.ty.to_latex(conf);

        out
    }
}
