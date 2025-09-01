use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    language::Language,
    {terms::TyApp, },
};

impl<Lang> RuleDescribe for TyApp<Lang>
where
    Lang: Language,
    ,
{
    fn rule() -> Rule {
        Rule::new(Symbol::app(Symbol::Term, Symbol::Type), "Type Application")
    }
}
