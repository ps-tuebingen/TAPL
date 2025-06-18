use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::{
    terms::{Fold, Term},
    types::Type,
};

impl<T, Ty> Parse for Fold<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
    <T as Parse>::ParseError: From<<Ty as Parse>::ParseError>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::fold_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Fold<T, Ty>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Fold Type", "fold Term"])?;
        let ty_rule = inner.remove(0);
        let ty = Ty::from_pair(ty_rule, ())?;
        let term_rule = inner.remove(0);
        let term = T::from_pair(term_rule, ())?;
        Ok(Fold::new(term, ty))
    }
}
