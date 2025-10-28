use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Cons};

impl<Lang> GrammarRuleDescribe for Cons<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(
                Keyword::Cons,
                Some(Symbol::Type),
                vec![Symbol::Term, Symbol::Term],
            ),
            "Cons",
        )
    }
}
