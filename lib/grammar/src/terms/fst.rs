use crate::{
    GrammarRule, GrammarRuleDescribe, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::Fst};

impl<Lang> GrammarRuleDescribe for Fst<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![Symbol::Term, SpecialChar::Dot.into(), Keyword::Fst.into()].into(),
            "First Projection",
        )
    }
}
