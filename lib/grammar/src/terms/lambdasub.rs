use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, terms::LambdaSub};

impl<Lang> GrammarRuleDescribe for LambdaSub<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::lam(Symbol::subty_annot(Symbol::Variable), Symbol::Term),
            "Lambda Sub",
        )
    }
}
