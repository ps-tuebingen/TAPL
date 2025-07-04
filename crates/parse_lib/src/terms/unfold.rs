use crate::{Parse, Rule, errors::ParserError, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::{
    terms::{Term, Unfold},
    types::Type,
};

impl<T, Ty> Parse for Unfold<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::unfold_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Unfold<T, Ty>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Unfold Type", "Unfold Term"])?;
        let ty_rule = inner.remove(0);
        let ty = Ty::from_pair(ty_rule, ())?;
        let term_rule = inner.remove(0);
        let term = T::from_pair(term_rule, ())?;
        Ok(Unfold::new(ty, term))
    }
}
