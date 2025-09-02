use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::Nil};

impl<Lang> Parse for Nil<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::nil_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Nil<Lang>, ParserError> {
        let ty_pair = pair_to_n_inner(p, vec!["Nil Type"])?.remove(0);
        let ty = Lang::Type::from_pair(ty_pair, ())?;

        Ok(Nil::new(ty))
    }
}
