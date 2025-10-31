use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::Unpack};

impl<Lang> GrammarRuleDescribe for Unpack<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Symbol::brack(Symbol::comma_sep(Symbol::Typevariable, Symbol::Variable)),
                SpecialChar::Equals.into(),
                Symbol::Term,
                Keyword::In.into(),
                Symbol::Term,
            ]
            .into(),
            "Unpack",
        )
    }
}
