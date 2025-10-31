use crate::{LatexConfig, LatexFmt};
use grammar::GrammarRule;

impl LatexFmt for GrammarRule {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (mut env_start, mut env_end) = conf.mathenv_strs();
        if conf.include_envs {
            env_start += "\n\\begin{array}{lr}";
            env_end += "\n\\end{array}";
        }
        conf.include_envs = false;

        format!(
            "{env_start} {}&{} {env_end}",
            self.symbol.to_latex(conf),
            self.description.to_latex(conf)
        )
    }
}
