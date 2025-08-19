use crate::{LatexConfig, LatexFmt};
use derivations::{Conclusion, SubtypeConclusion, TypingConclusion};
use syntax::{terms::Term, types::Type};

impl<T, Ty> LatexFmt for Conclusion<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Conclusion::Typing(conc) => conc.to_latex(conf),
            Conclusion::Subtyping(conc) => conc.to_latex(conf),
        }
    }
}

impl<T, Ty> LatexFmt for TypingConclusion<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;

        let env_str = self.env.to_latex(conf);
        conf.include_envs = false;
        let term_str = self.term.to_latex(conf);
        conf.include_envs = false;
        let ty_str = self.ty.to_latex(conf);

        format!("{env_start} {env_str} \\vdash {term_str} : {ty_str} {env_end}")
    }
}

impl<Ty> LatexFmt for SubtypeConclusion<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;

        let env_str = self.env.to_latex(conf);
        conf.include_envs = false;
        let sub_str = self.sub.to_latex(conf);
        conf.include_envs = false;
        let sup_str = self.sup.to_latex(conf);

        format!("{env_start} {env_str} \\vdash {sub_str} <: {sup_str} {env_end}")
    }
}
