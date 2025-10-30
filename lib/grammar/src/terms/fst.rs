use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::Fst};

impl<Lang> GrammarRuleDescribe for Fst<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![Symbol::Term, SpecialChar::Dot.into(), Keyword::Fst.into()].into(),
            "First Projection",
        )
    }
}
