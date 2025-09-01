use crate::{LatexConfig, LatexFmt};
use syntax::{language::Language, program::Program};

impl<Lang> LatexFmt for Program<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;

        format!(
            "{env_start}{} \\\\ \\text{{def main }} {};{env_end}",
            self.definitions
                .iter()
                .map(|def| def.to_latex(conf))
                .collect::<Vec<String>>()
                .join("\n\\\\\n"),
            self.main.to_latex(conf)
        )
    }
}
