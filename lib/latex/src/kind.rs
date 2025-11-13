use super::{LatexConfig, LatexFmt};
use syntax::kinds::Kind;

impl LatexFmt for Kind {
    fn to_latex(&self, _conf: &mut LatexConfig) -> String {
        match self {
            Self::Star => "*".to_owned(),
            Self::Arrow(from, to) => {
                format!(
                    "{} \\Rightarrow {}",
                    from.to_latex(_conf),
                    to.to_latex(_conf)
                )
            }
        }
    }
}
