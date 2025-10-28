use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::SomeCase};

impl<Lang> GrammarRuleDescribe for SomeCase<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::case(vec![
                Symbol::pt(Symbol::ctor_pt(Keyword::Nothing, 0), Symbol::Term),
                Symbol::pt(Symbol::ctor_pt(Keyword::Something, 1), Symbol::Term),
            ]),
            "Option Case",
        )
    }
}
