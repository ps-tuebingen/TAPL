use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::{
    terms::{Right, Term},
    types::Type,
};

impl<T, Ty> Parse for Right<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
    <T as Parse>::ParseError: From<<Ty as Parse>::ParseError>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::right_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Right<T, Ty>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Inr Argument", "Inr Type"])?;

        let arg_pair = inner.remove(0);
        let arg_term = T::from_pair(
            pair_to_n_inner(arg_pair, vec!["Paren Term Inner"])?.remove(0),
            (),
        )?;

        let ty_pair = inner.remove(0);
        let ty = Ty::from_pair(ty_pair, ())?;

        Ok(Right::new(arg_term, ty))
    }
}
