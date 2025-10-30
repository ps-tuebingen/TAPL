use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::VariantCase};

impl<Lang> GrammarRuleDescribe for VariantCase<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Case.into(),
                Symbol::Term,
                Keyword::Of.into(),
                Symbol::many(vec![
                    SpecialChar::AngBrackO.into(),
                    Symbol::Many(Box::new(
                        vec![Symbol::Label, SpecialChar::Equals.into(), Symbol::Variable].into(),
                    )),
                    SpecialChar::AngBrackC.into(),
                    SpecialChar::DoubleArrow.into(),
                    Symbol::Term,
                ]),
            ]
            .into(),
            "Variant Case",
        )
    }
}
