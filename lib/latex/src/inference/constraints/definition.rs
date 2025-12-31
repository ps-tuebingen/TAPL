use crate::{LatexConfig, LatexFmt};
use inference::DefConstraints;
use syntax::language::Language;

impl<Lang> LatexFmt for DefConstraints<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let mut constr_strs = Vec::with_capacity(self.constraints.len());
        for constr in &self.constraints {
            constr_strs.push(constr.to_latex(conf));
        }
        format!(
            "{}\n\\\\\n{}",
            constr_strs.join("\n\\\\\n"),
            self.ret_ty.to_latex(conf)
        )
    }
}
