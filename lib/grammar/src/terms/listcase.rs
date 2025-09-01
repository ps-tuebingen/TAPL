use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::ListCase};

impl<Lang> RuleDescribe for ListCase<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::case(vec![
                Symbol::pt(Symbol::ctor_pt(Keyword::Nil, 0), Symbol::Term),
                Symbol::pt(Symbol::ctor_pt(Keyword::Cons, 2), Symbol::Term),
            ]),
            "Cast",
        )
    }
}
