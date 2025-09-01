use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Unpack};

impl<Lang> LatexFmt for Unpack<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{let}} \\{{{},{}\\}}={} \\text{{in}} {}",
            self.ty_name.to_latex(conf),
            self.term_name.to_latex(conf),
            self.bound_term.to_latex(conf),
            self.in_term.to_latex(conf)
        )
    }
}
