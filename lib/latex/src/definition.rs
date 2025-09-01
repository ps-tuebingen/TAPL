use crate::{LatexConfig, LatexFmt};
use syntax::{definition::Definition, language::Language};

impl<Lang> LatexFmt for Definition<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;
        format!(
            "{env_start}\\text{{def }} {}::{} := {};{env_end}",
            self.name,
            self.annot.to_latex(conf),
            self.body.to_latex(conf)
        )
    }
}
