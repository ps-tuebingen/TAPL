use crate::{
    GrammarRule, GrammarRuleDescribe, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::VariantCase};

impl<Lang> GrammarRuleDescribe for VariantCase<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![
                Keyword::Case.into(),
                Symbol::Term,
                Keyword::Of.into(),
                Symbol::many(vec![
                    Symbol::angbrack(vec![Symbol::Many(Box::new(
                        vec![Symbol::Label, SpecialChar::Equals.into(), Symbol::Variable].into(),
                    ))]),
                    SpecialChar::DoubleArrow.into(),
                    Symbol::Term,
                ]),
            ]
            .into(),
            "Variant Case",
        )
    }
}
