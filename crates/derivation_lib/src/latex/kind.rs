use super::{LatexConfig, LatexFmt};
use syntax::kinds::Kind;

impl LatexFmt for Kind {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Kind::Star => "*".to_owned(),
            Kind::Arrow(from, to) => {
                format!("{} \\Rightarrow {}", from.to_latex(conf), to.to_latex(conf))
            }
        }
    }
}
