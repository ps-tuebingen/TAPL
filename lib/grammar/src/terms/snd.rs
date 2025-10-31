use crate::{
    GrammarRule, GrammarRuleDescribe, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::Snd};

impl<Lang> GrammarRuleDescribe for Snd<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![Symbol::Term, SpecialChar::Dot.into(), Keyword::Snd.into()].into(),
            "Second Projection",
        )
    }
}
