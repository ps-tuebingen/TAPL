use crate::{LatexConfig, LatexFmt};
use inference::{DefSubst, ProgSubst};
use syntax::language::Language;

impl<Lang> LatexFmt for ProgSubst<Lang>
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
        let mut def_strs = Vec::with_capacity(self.def_substs.len());
        for (name, def_subst) in &self.def_substs {
            let def_str = def_subst.to_latex(conf);
            def_strs.push(format!("{name}:\n\\\\{def_str}"));
        }
        format!(
            "{text_start}Generated Substitution{text_end}\n{}\nmain:\n\\\\{}",
            def_strs.join("\\\\\n"),
            self.main_subst.to_latex(conf)
        )
    }
}

impl<Lang> LatexFmt for DefSubst<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;

        let mut subst_strs = Vec::with_capacity(self.ty_vars.len());
        for (v, ty) in &self.ty_vars {
            let ty_str = ty.to_latex(conf);
            conf.include_envs = false;
            subst_strs.push(format!("{v} \\mapsto {ty_str}"));
        }

        format!(
            "{env_start}{}\n\\\\\n{}{env_end}",
            subst_strs.join("\n\\\\\n"),
            self.ty_no_subst.to_latex(conf)
        )
    }
}
