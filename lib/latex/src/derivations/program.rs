use crate::{LatexConfig, LatexFmt};
use derivations::ProgramDerivation;
use syntax::language::Language;

impl<Lang> LatexFmt for ProgramDerivation<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let mut def_strs = vec![];
        let old_inc = conf.include_envs;
        for def in &self.def_derivations {
            def_strs.push(def.to_latex(conf));
            conf.include_envs = old_inc;
        }
        def_strs.push(self.main_derivation.to_latex(conf));
        def_strs.join("\n\\quad \\\\ \\quad \\\\\n")
    }
}
