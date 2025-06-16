use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::{
    terms::{Exception, Term},
    types::Type,
};

impl<T, Ty> Parse for Exception<T, Ty>
where
    T: Term + Parse,
    Ty: Type + Parse<LeftRecArg = ()>,
    <T as Parse>::ParseError: From<<Ty as Parse>::ParseError>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    fn rule() -> Rule {
        Rule::err_term
    }

    fn from_pair(
        p: Pair<'_, Rule>,
        _: Self::LeftRecArg,
    ) -> Result<Exception<T, Ty>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Error Type"])?;
        let ty_rule = inner.remove(0);
        let ty = Ty::from_pair(ty_rule, ())?;
        Ok(Exception::new(ty))
    }
}
