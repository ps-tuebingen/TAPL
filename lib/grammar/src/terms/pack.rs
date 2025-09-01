use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    language::Language,
    {terms::Pack, },
};

impl<Lang> RuleDescribe for Pack<Lang>
where
    Lang: Language,
    ,
{
    fn rule() -> Rule {
        Rule::new(Symbol::pack(Symbol::Term), "Package")
    }
}
