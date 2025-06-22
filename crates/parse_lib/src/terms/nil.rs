use crate::{Parse, Rule, errors::ParserError, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::{
    terms::{Nil, Term},
    types::Type,
};

impl<T, Ty> Parse for Nil<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::nil_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Nil<T, Ty>, ParserError> {
        let ty_pair = pair_to_n_inner(p, vec!["Nil Type"])?.remove(0);
        let ty = Ty::from_pair(ty_pair, ())?;

        Ok(Nil::new(ty))
    }
}
