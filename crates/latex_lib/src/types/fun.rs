use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Fun, Type};

impl<Ty> LatexFmt for Fun<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "{}\\rightarrow {}",
            self.from.to_latex(conf),
            self.to.to_latex(conf)
        )
    }
}
