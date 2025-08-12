use crate::{Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{
    terms::{Nothing, Term},
    types::Type,
};

impl<T, Ty> Parse for Nothing<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::none_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Nothing<T, Ty>, ParserError> {
        let ty_pair = pair_to_n_inner(p, vec!["Nothing Type"])?.remove(0);
        let ty = Ty::from_pair(ty_pair, ())?;
        Ok(Nothing::new(ty))
    }
}
