use crate::{Parse, Rule, errors::ParserError, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::{
    terms::{Pack, Term},
    types::Type,
};

impl<T, Ty> Parse for Pack<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::pack_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Pack<T, Ty>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Packed Type", "Packed Term", "Pack Type"])?;
        let packed_rule = inner.remove(0);
        let packed_ty = Ty::from_pair(packed_rule, ())?;

        let term_rule = inner.remove(0);
        let term = T::from_pair(term_rule, ())?;

        let pack_rule = inner.remove(0);
        let pack_ty = Ty::from_pair(pack_rule, ())?;

        Ok(Pack::new(packed_ty, term, pack_ty))
    }
}
