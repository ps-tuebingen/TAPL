use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    language::Language,
    {terms::Unpack, },
};

impl<Lang> RuleDescribe for Unpack<Lang>
where
    Lang: Language,
    ,
{
    fn rule() -> Rule {
        Rule::new(Symbol::unpack(), "Unpack")
    }
}
