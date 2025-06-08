use super::LatexFmt;
use syntax::untyped::Untyped;

impl LatexFmt for Untyped {
    fn to_latex(&self) -> String {
        "".to_owned()
    }
}
