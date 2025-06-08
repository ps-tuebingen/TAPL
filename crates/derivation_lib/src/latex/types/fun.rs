use super::super::LatexFmt;
use syntax::types::{Fun, Type};

impl<Ty> LatexFmt for Fun<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "{}\\rightarrow {}",
            self.from.to_latex(),
            self.to.to_latex()
        )
    }
}
