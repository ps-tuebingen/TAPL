use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, values::Lambda};

impl<Lang> GrammarRuleDescribe for Lambda<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::lam(Symbol::ty_annot(Symbol::Variable), Symbol::Term),
            "Lambda Abstraction",
        )
    }
}
