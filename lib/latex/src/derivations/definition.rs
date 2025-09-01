use crate::{LatexConfig, LatexFmt};
use derivations::DefinitionDerivation;
use syntax::language::Language;

impl<Lang> LatexFmt for DefinitionDerivation<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (mut env_start, mut env_end) = conf.mathenv_strs();
        if !conf.use_frac_array && conf.include_envs {
            env_start = "\\begin{prooftree}".to_owned();
            env_end = "\\end{prooftree}".to_owned();
        };

        conf.include_envs = false;
        let body_str = self.body_derivation.to_latex(conf);
        conf.include_envs = false;
        let ty_str = self.body_derivation.ret_ty().to_latex(conf);

        if conf.use_frac_array {
            format!(
                "{env_start}\n\\frac{{ {body_str} }}{{ \\vdash {}:{} }}\n{env_end}",
                self.name, ty_str
            )
        } else {
            format!(
                "{env_start}\n{body_str}\n\\UnaryInfC{{$\\vdash {}:{}$}}\n{env_end}",
                self.name, ty_str
            )
        }
    }
}
