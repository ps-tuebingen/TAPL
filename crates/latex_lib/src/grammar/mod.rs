use crate::{LatexConfig, LatexFmt};
use grammar::{Grammar, GrammarDescribe, LanguageGrammar};
use syntax::kinds::Kind;

mod keyword;
mod rule;
mod special_char;
mod symbol;

impl LatexFmt for Grammar {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = if conf.include_envs {
            ("\\[\\begin{array}{lcrr}", "\\end{array}\\]")
        } else {
            ("", "")
        };
        conf.include_envs = false;

        let mut alt_strs = vec![];

        for alt in self.alternatives.iter() {
            alt_strs.push(format!("&\\mid &{}\\\\", alt.to_latex(conf)));
        }

        format!(
            "{}\n{} & := & & {} \\\\ {} \n{}",
            env_start,
            self.symbol.to_latex(conf),
            self.description.to_latex(conf),
            alt_strs.join("\n"),
            env_end
        )
    }
}

impl LatexFmt for LanguageGrammar {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = if conf.include_envs {
            ("\\[\\begin{array}{lcrr}", "\\end{array}\\]")
        } else {
            ("", "")
        };
        conf.include_envs = false;
        let term_grammar = self.term_grammar.to_latex(conf);

        conf.include_envs = false;
        let type_grammar = self.type_grammar.to_latex(conf);

        conf.include_envs = false;
        let value_grammar = self.value_grammar.to_latex(conf);

        conf.include_envs = false;
        let kind_grammar = if self.include_kinds {
            Kind::grammar().to_latex(conf)
        } else {
            "".to_owned()
        };

        format!(
            "{env_start}{term_grammar}\n\\\\\n{type_grammar}\n\\\\\n{value_grammar}\n\\\\\n{kind_grammar}{env_end}"
        )
    }
}
