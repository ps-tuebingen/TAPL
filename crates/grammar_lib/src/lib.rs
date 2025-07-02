mod grammar;
mod rule;
mod symbols;

mod terms;
mod types;
mod untyped;
mod values;

pub use grammar::{Grammar, LanguageGrammar};
pub use rule::Rule;
pub use symbols::Symbol;

pub trait GrammarDescribe {
    fn grammar() -> Grammar;
}

pub trait LanguageDescribe {
    fn grammars() -> LanguageGrammar;
}

pub trait RuleDescribe {
    fn rule() -> Rule;
}
