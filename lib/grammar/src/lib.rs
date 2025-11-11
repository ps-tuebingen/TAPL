use std::{collections::HashSet, fmt};
use syntax::kinds::Kind;

mod grammar;
mod rules;
pub mod symbols;

mod kinds;
mod terms;
mod types;
mod untyped;
mod values;

pub use grammar::Grammar;
pub use rules::{ConclusionRule, DerivationRule, GrammarRule};
pub use symbols::Symbol;

pub trait GrammarDescribe {
    fn grammar() -> Grammar;
}

pub trait LanguageDescribe {
    fn grammars() -> LanguageGrammar;
    fn rules() -> LanguageRules;
}

pub trait GrammarRuleDescribe {
    fn rule() -> GrammarRule;
}

#[derive(Debug)]
pub struct LanguageRules {
    pub typing: HashSet<DerivationRule>,
    pub subtyping: HashSet<DerivationRule>,
    pub kinding: HashSet<DerivationRule>,
    pub normalizing: HashSet<DerivationRule>,
    pub eval: HashSet<DerivationRule>,
}
/// Grammar of a language
#[derive(Debug)]
pub struct LanguageGrammar {
    /// The grammar of terms
    pub term_grammar: Grammar,
    /// The grammar of types
    pub type_grammar: Grammar,
    /// The grammar of values
    pub value_grammar: Grammar,
    /// Is the language kinded?
    pub include_kinds: bool,
}

impl fmt::Display for LanguageGrammar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.term_grammar)?;
        writeln!(f, "{}", self.type_grammar)?;
        writeln!(f, "{}", self.value_grammar)?;
        if self.include_kinds {
            writeln!(f, "{}", Kind::grammar())?;
        }
        Ok(())
    }
}

impl fmt::Display for LanguageRules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Typing Rules")?;
        for rule in self.typing.iter() {
            writeln!(f, "{rule}")?;
        }

        writeln!(f, "Subtyping Rules")?;
        for rule in self.subtyping.iter() {
            writeln!(f, "{rule}")?;
        }

        writeln!(f, "Kinding Rules")?;
        for rule in self.kinding.iter() {
            writeln!(f, "{rule}")?;
        }

        writeln!(f, "Normalizing Rules")?;
        for rule in self.normalizing.iter() {
            writeln!(f, "{rule}")?;
        }

        writeln!(f, "Evaluation Rules")?;
        for rule in self.eval.iter() {
            writeln!(f, "{rule}")?;
        }
        Ok(())
    }
}
