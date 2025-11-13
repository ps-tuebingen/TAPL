use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::TyApp};

impl<Lang> Parse for TyApp<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = Lang::Term;

    const RULE: Rule = Rule::tyapp;

    fn from_pair(p: Pair<'_, Rule>, t: Self::LeftRecArg) -> Result<Self, ParserError> {
        let ty_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
        let ty = Lang::Type::from_pair(ty_rule, ())?;
        Ok(Self::new(t, ty))
    }
}
