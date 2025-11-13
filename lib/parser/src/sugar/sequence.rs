use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{
    language::Language,
    terms::{App, Lambda},
    types::Unit,
};

pub struct Sequence<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    fst: Lang::Term,
    snd: Lang::Term,
}

impl<Lang> Sequence<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    pub fn to_term(self) -> Lang::Term
    where
        App<Lang>: Into<Lang::Term>,
        Lambda<Lang>: Into<Lang::Term>,
        Unit<Lang>: Into<Lang::Type>,
    {
        App::new(Lambda::new("_", Unit::new(), self.snd), self.fst).into()
    }
}

impl<Lang> Parse for Sequence<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = Lang::Term;

    const RULE: Rule = Rule::sequence;

    fn from_pair(p: Pair<'_, Rule>, t: Self::LeftRecArg) -> Result<Self, ParserError> {
        let term_rule = pair_to_n_inner(p, vec!["Sequence Second Term"])?.remove(0);
        let term = Lang::Term::from_pair(term_rule, ())?;
        Ok(Self { fst: t, snd: term })
    }
}
