use crate::{Parse, Rule, errors::ParserError, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::{
    terms::{IsNil, Term},
    types::Type,
};

impl<T, Ty> Parse for IsNil<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();
    const RULE: Rule = Rule::isnil_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<IsNil<T, Ty>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["IsNil Type", "IsNil Argument"])?;

        let ty_pair = inner.remove(0);
        let ty = Ty::from_pair(ty_pair, ())?;

        let term_pair = inner.remove(0);
        let term = T::from_pair(term_pair, ())?;

        Ok(IsNil::new(term, ty))
    }
}
