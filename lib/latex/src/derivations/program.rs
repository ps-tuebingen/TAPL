use crate::{LatexConfig, LatexFmt};
use derivations::ProgramDerivation;
use syntax::{terms::Term, types::Type};

impl<T, Ty> LatexFmt for ProgramDerivation<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let mut def_strs = vec![];
        let old_inc = conf.include_envs;
        for def in self.def_derivations.iter() {
            def_strs.push(def.to_latex(conf));
            conf.include_envs = old_inc;
        }
        def_strs.push(self.main_derivation.to_latex(conf));
        def_strs.join("\n\\quad \\\\ \\quad \\\\\n")
    }
}
