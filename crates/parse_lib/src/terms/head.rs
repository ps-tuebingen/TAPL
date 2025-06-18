use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::{
    terms::{Head, Term},
    types::Type,
};

impl<T, Ty> Parse for Head<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
    <T as Parse>::ParseError: From<<Ty as Parse>::ParseError>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::head_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Head<T, Ty>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Head Type", "Head Argument"])?;

        let ty_rule = inner.remove(0);
        let ty = Ty::from_pair(ty_rule, ())?;

        let term_pair = inner.remove(0);
        let term = T::from_pair(term_pair, ())?;

        Ok(Head::new(term, ty))
    }
}
