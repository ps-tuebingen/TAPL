use crate::{Parse, Rule, errors::ParserError, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::{
    terms::{Ascribe, Term},
    types::Type,
};

impl<T, Ty> Parse for Ascribe<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = T;

    const RULE: Rule = Rule::ascription;

    fn from_pair(p: Pair<'_, Rule>, t: Self::LeftRecArg) -> Result<Ascribe<T, Ty>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Ascribed Type"])?;

        let ty_rule = inner.remove(0);
        let ty = Ty::from_pair(ty_rule, ())?;

        Ok(Ascribe::new(t, ty))
    }
}
