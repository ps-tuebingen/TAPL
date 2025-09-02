use crate::{GroupParse, Parse, Rule};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::Tuple};

impl<Lang> Parse for Tuple<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();
    const RULE: Rule = Rule::tuple_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Tuple<Lang>, ParserError> {
        let mut terms = vec![];
        for p in p.into_inner() {
            let p_term = Lang::Term::from_pair(p, ())?;
            terms.push(p_term);
        }
        Ok(Tuple::new(terms))
    }
}
