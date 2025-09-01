use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    language::Language,
    {language::Language, terms::Try},
};

impl<Lang> RuleDescribe for Try<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::tryt(), "Variant Case")
    }
}
