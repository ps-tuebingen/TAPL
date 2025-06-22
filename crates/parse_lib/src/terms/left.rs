use crate::{Parse, Rule, errors::ParserError, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::{
    terms::{Left, Term},
    types::Type,
};

impl<T, Ty> Parse for Left<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::left_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Left<T, Ty>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Inl Argument", "Inl Type"])?;

        let arg_pair = inner.remove(0);
        let arg_term = T::from_pair(
            pair_to_n_inner(arg_pair, vec!["Paren Term Inner"])?.remove(0),
            (),
        )?;

        let ty_pair = inner.remove(0);
        let ty = Ty::from_pair(ty_pair, ())?;

        Ok(Left::new(arg_term, ty))
    }
}
