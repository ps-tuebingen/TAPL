use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::Cons};

impl<Lang> GrammarRuleDescribe for Cons<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Cons.into(),
                Symbol::sqbrack(Symbol::Type),
                Symbol::paren(Symbol::comma_sep(Symbol::Term, Symbol::Term)),
            ]
            .into(),
            "Cons",
        )
    }
}
