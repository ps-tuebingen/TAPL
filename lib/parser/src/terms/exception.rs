use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::Exception};

impl<Lang> Parse for Exception<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::err_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Exception<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Error Type"])?;
        let ty_rule = inner.remove(0);
        let ty = Lang::Type::from_pair(ty_rule, ())?;
        Ok(Exception::new(ty))
    }
}
