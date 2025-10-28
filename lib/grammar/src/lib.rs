mod derivation_rule;
mod grammar;
mod rule;
pub mod symbols;

mod kinds;
mod terms;
mod types;
mod untyped;
mod values;

pub use derivation_rule::{ConclusionRule, DerivationRule, LanguageRules};
pub use grammar::{Grammar, LanguageGrammar};
pub use rule::Rule;
pub use symbols::Symbol;

pub trait GrammarDescribe {
    fn grammar() -> Grammar;
}

pub trait LanguageDescribe {
    fn grammars() -> LanguageGrammar;
    fn rules() -> LanguageRules;
}

pub trait GrammarRuleDescribe {
    fn rule() -> Rule;
}
