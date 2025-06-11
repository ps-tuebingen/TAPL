use super::super::{LatexConfig, LatexFmt};
use syntax::{
    terms::{Term, Unpack},
    types::Type,
};

impl<T, Ty> LatexFmt for Unpack<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{let}} \\{{{},{}\\}}={} \\text{{in}} {}",
            self.ty_name,
            self.term_name,
            self.bound_term.to_latex(conf),
            self.in_term.to_latex(conf)
        )
    }
}
