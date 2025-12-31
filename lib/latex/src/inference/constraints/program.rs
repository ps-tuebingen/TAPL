use crate::{LatexConfig, LatexFmt};
use inference::ProgramConstraints;
use syntax::language::Language;

impl<Lang> LatexFmt for ProgramConstraints<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (text_start, text_end) = if conf.include_envs {
            ("", "")
        } else {
            ("\\text{", "}")
        };
        let mut def_strs = Vec::with_capacity(self.def_constraints.len());
        for (name, def_constrs) in &self.def_constraints {
            let def_str = def_constrs.to_latex(conf);
            def_strs.push(format!("{name}:\n\\\\{def_str}"));
        }
        format!(
            "{text_start}Generated Constraints{text_end}\n{}\nmain:\n\\\\{}",
            def_strs.join("\\\\\n"),
            self.main_constraints.to_latex(conf)
        )
    }
}
