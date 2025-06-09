use super::{LatexConfig, LatexFmt};
use syntax::untyped::Untyped;

impl LatexFmt for Untyped {
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        "".to_owned()
    }
}
