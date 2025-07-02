use crate::{LatexConfig, LatexFmt};
use derivation::Conclusion;
use syntax::{terms::Term, types::Type};

impl<T, Ty> LatexFmt for Conclusion<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;

        let mut out = env_start;

        out += &self.env.to_latex(conf);
        out += " \\vdash ";
        out += &self.term.to_latex(conf);
        out += " : ";
        out += &self.ty.to_latex(conf);
        out += &env_end;

        out
    }
}
