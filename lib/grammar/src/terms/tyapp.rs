use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, terms::TyApp};

impl<Lang> GrammarRuleDescribe for TyApp<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::app(Symbol::Term, Symbol::Type), "Type Application")
    }
}
