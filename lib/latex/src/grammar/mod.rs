use crate::{LatexConfig, LatexFmt};
use grammar::{Grammar, GrammarDescribe, LanguageGrammar, LanguageRules};
use syntax::kinds::Kind;

mod derivation_rule;
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

impl LatexFmt for LanguageRules {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;
        let mut ty_str = self
            .typing
            .iter()
            .map(|rule| rule.to_latex(conf))
            .collect::<Vec<_>>()
            .join("\n\n");
        if !ty_str.is_empty() {
            ty_str = format!("Typing\n{ty_str}")
        }
        let mut subty_str = self
            .subtyping
            .iter()
            .map(|rule| rule.to_latex(conf))
            .collect::<Vec<_>>()
            .join("\n\n");
        if !subty_str.is_empty() {
            subty_str = format!("Subtyping\n{subty_str}")
        }
        let mut kind_str = self
            .kinding
            .iter()
            .map(|rule| rule.to_latex(conf))
            .collect::<Vec<_>>()
            .join("\n\n");
        if !kind_str.is_empty() {
            kind_str = format!("Kinding\n{kind_str}")
        }
        let mut norm_str = self
            .normalizing
            .iter()
            .map(|rule| rule.to_latex(conf))
            .collect::<Vec<_>>()
            .join("\n\n");
        if !norm_str.is_empty() {
            norm_str = format!("Normalizing\n{kind_str}")
        }
        let mut eval_str = self
            .eval
            .iter()
            .map(|rule| rule.to_latex(conf))
            .collect::<Vec<_>>()
            .join("\n\n");
        if !eval_str.is_empty() {
            eval_str = format!("Evaluation\n{kind_str}")
        }
        format!(
            "{env_start}{ty_str}\n\n{subty_str}\n\n{kind_str}\n\n{norm_str}\n\n{eval_str}{env_end}"
        )
    }
}
