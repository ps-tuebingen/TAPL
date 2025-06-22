use crate::{Parse, Rule, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::{
    terms::{Cast, Term},
    types::Type,
};

impl<T, Ty> Parse for Cast<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
    <T as Parse>::ParseError: From<<Ty as Parse>::ParseError>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = T;
    const RULE: Rule = Rule::cast;

    fn from_pair(p: Pair<'_, Rule>, t: Self::LeftRecArg) -> Result<Cast<T, Ty>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Cast Type"])?;
        let ty_rule = inner.remove(0);
        let ty = Ty::from_pair(ty_rule, ())?;
        Ok(Cast::new(t, ty))
    }
}
