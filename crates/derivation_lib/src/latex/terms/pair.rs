use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Pair, Term};

impl<T> LatexFmt for Pair<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\{{ {}, {} \\}}",
            self.fst.to_latex(conf),
            self.snd.to_latex(conf)
        )
    }
}
