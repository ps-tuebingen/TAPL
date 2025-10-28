use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, terms::Ascribe};

impl<Lang> GrammarRuleDescribe for Ascribe<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::ty_annot(Symbol::Term), "Ascription")
    }
}
