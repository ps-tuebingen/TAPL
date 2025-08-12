use super::{LatexConfig, LatexFmt};
use syntax::{env::Environment, types::Type};

impl<Ty> LatexFmt for Environment<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;

        let def_strs = self
            .definitions
            .keys()
            .map(|name| {
                let out = name.to_latex(conf);
                conf.include_envs = false;
                out
            })
            .collect::<Vec<String>>();

        let def_sep = if def_strs.is_empty() { "" } else { ";" };

        let var_strs = self
            .var_bindings
            .iter()
            .map(|(var, ty)| {
                let out = format!("{}:{}", var.to_latex(conf), ty.to_latex(conf));
                conf.include_envs = false;
                out
            })
            .collect::<Vec<String>>();

        let var_sep = if var_strs.is_empty() { "" } else { ";" };

        let tyvar_strs = self
            .tyvar_bindings
            .iter()
            .map(|(var, knd)| {
                let out = format!("{}::{}", var.to_latex(conf), knd.to_latex(conf));
                conf.include_envs = false;
                out
            })
            .collect::<Vec<String>>();

        let tyvar_sep = if tyvar_strs.is_empty() { "" } else { ";" };

        let super_strs = self
            .tyvar_super
            .iter()
            .map(|(var, ty)| {
                let out = format!("{}<:{}", var.to_latex(conf), ty.to_latex(conf));
                conf.include_envs = false;
                out
            })
            .collect::<Vec<String>>();

        let super_sep = if super_strs.is_empty() { "" } else { ";" };

        let loc_strs = self
            .location_bindings
            .iter()
            .map(|(loc, ty)| {
                let out = format!("{loc}:{}", ty.to_latex(conf));
                conf.include_envs = false;
                out
            })
            .collect::<Vec<String>>();

        format!(
            "{env_start}{}{def_sep}{}{var_sep}{}{tyvar_sep}{}{super_sep}{}{env_end}",
            def_strs.join(", "),
            var_strs.join(", "),
            tyvar_strs.join(", "),
            super_strs.join(", "),
            loc_strs.join(", ")
        )
    }
}
