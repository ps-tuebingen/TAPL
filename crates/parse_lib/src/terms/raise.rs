use crate::{Parse, Rule, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::{
    terms::{Raise, Term},
    types::Type,
};

impl<T, Ty> Parse for Raise<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
    <T as Parse>::ParseError: From<<Ty as Parse>::ParseError>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::raise_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Raise<T, Ty>, Self::ParseError> {
        let mut inner = pair_to_n_inner(
            p,
            vec![
                "Raise Continuation Type",
                "Raise Exception Type",
                "Raise Term",
            ],
        )?;
        let cont_ty_rule = inner.remove(0);
        let cont_ty = Ty::from_pair(cont_ty_rule, ())?;
        let ex_ty_rule = inner.remove(0);
        let ex_ty = Ty::from_pair(ex_ty_rule, ())?;
        let catch_rule = inner.remove(0);
        let catch_term = T::from_pair(catch_rule, ())?;
        Ok(Raise::new(catch_term, cont_ty, ex_ty))
    }
}
