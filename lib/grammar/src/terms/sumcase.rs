use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword, symbols::SpecialChar};
use syntax::{language::Language, terms::SumCase};

impl<Lang> GrammarRuleDescribe for SumCase<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Case.into(),
                Symbol::Term,
                Keyword::Of.into(),
                Keyword::Left.into(),
                Symbol::paren(vec![Symbol::Variable]),
                SpecialChar::DoubleArrow.into(),
                Symbol::Term,
                SpecialChar::Pipe.into(),
                Keyword::Right.into(),
                Symbol::paren(vec![Symbol::Variable]),
                SpecialChar::DoubleArrow.into(),
                Symbol::Term,
            ]
            .into(),
            "Sum Case",
        )
    }
}
