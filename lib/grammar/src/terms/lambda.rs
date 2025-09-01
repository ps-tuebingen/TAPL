use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, terms::Lambda};

impl<Lang> RuleDescribe for Lambda<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::lam(Symbol::ty_annot(Symbol::Variable), Symbol::Term),
            "Lambda Abstracion",
        )
    }
}
