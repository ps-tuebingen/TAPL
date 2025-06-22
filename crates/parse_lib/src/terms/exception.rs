use crate::{Parse, Rule, errors::ParserError, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::{
    terms::{Exception, Term},
    types::Type,
};

impl<T, Ty> Parse for Exception<T, Ty>
where
    T: Term + Parse,
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::err_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Exception<T, Ty>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Error Type"])?;
        let ty_rule = inner.remove(0);
        let ty = Ty::from_pair(ty_rule, ())?;
        Ok(Exception::new(ty))
    }
}
