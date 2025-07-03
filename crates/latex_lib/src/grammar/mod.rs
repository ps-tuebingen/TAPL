use crate::{LatexConfig, LatexFmt};
use grammar::{Grammar, GrammarDescribe, LanguageGrammar};
use syntax::kinds::Kind;

mod keyword;
mod rule;
mod special_char;
mod symbol;

impl LatexFmt for Grammar {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;

        let mut alt_strs = vec![];

        for alt in self.alternatives.iter() {
            alt_strs.push(format!("&{}\\\\", alt.to_latex(conf)));
        }

        format!(
            "{}\n\\begin{{array}}{{lcr}}\n{} & \\coloneq & {} \\\\ {} \\end{{array}}\n{}",
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
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;

        let mut grammars = vec![
            self.term_grammar.to_latex(conf),
            self.type_grammar.to_latex(conf),
            self.value_grammar.to_latex(conf),
        ];
        if self.include_kinds {
            grammars.push(Kind::grammar().to_latex(conf));
        }

        format!("{env_start}{}{env_end}", grammars.join("\\\\ \\quad \\\\"))
    }
}
