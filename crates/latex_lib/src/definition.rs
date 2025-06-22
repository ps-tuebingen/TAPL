use crate::{LatexConfig, LatexFmt};
use syntax::{definition::Definition, terms::Term, types::Type};

impl<T, Ty> LatexFmt for Definition<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = if conf.include_envs {
            ("\\[", "\\]")
        } else {
            ("", "")
        };

        conf.include_envs = false;
        format!(
            "{env_start}\\text{{def }} {}::{}\\coloneq {}{env_end}",
            self.name,
            self.annot.to_latex(conf),
            self.body.to_latex(conf)
        )
    }
}
