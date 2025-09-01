use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Snd};

impl<Lang> RuleDescribe for Snd<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::dot(Keyword::Snd.into()), "Second Projection")
    }
}
