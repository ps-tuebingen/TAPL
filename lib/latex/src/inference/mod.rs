use super::{LatexConfig, LatexFmt};
use inference::ProgTypes;
use syntax::language::Language;

mod constraints;
mod subst;

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
            def_strs.push(format!("{name_str}:{ty_str}"));
        }
        format!(
            "{env_start}{}\\main:{}{env_end}",
            def_strs.join("\\"),
            self.main_ty.to_latex(conf)
        )
    }
}
