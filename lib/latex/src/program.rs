use crate::{LatexConfig, LatexFmt};
use inference::ProgTypes;
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

impl<Lang> LatexFmt for ProgTypes<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;
        let mut def_strs = Vec::with_capacity(self.def_tys.len());
        for (name, ty) in &self.def_tys {
            let name_str = name.to_latex(conf);
            conf.include_envs = false;
            let ty_str = ty.to_latex(conf);
            conf.include_envs = false;
            def_strs.push(format!("{}:{}", name_str, ty_str))
        }
        format!(
            "{env_start}{}\\main:{}{env_end}",
            def_strs.join("\\"),
            self.main_ty.to_latex(conf)
        )
    }
}
