use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::SumCase};

impl<Lang> GrammarRuleDescribe for SumCase<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::case(vec![
                Symbol::pt(Symbol::ctor_pt(Keyword::Left, 1), Symbol::Term),
                Symbol::pt(Symbol::ctor_pt(Keyword::Right, 1), Symbol::Term),
            ]),
            "Sum Case",
        )
    }
}
