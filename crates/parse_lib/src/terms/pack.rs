use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::{
    terms::{Pack, Term},
    types::Type,
};

impl<T, Ty> Parse for Pack<T, Ty>
where
    T: Term + Parse,
    Ty: Type + Parse,
    <T as Parse>::ParseError: From<<Ty as Parse>::ParseError>,
{
    type ParseError = <T as Parse>::ParseError;

    fn rule() -> Rule {
        Rule::pack_term
    }

    fn from_pair(p: Pair<'_, Rule>) -> Result<Pack<T, Ty>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Packed Type", "Packed Term", "Pack Type"])?;
        let packed_rule = inner.remove(0);
        let packed_ty = Ty::from_pair(packed_rule)?;

        let term_rule = inner.remove(0);
        let term = T::from_pair(term_rule)?;

        let pack_rule = inner.remove(0);
        let pack_ty = Ty::from_pair(pack_rule)?;

        Ok(Pack::new(packed_ty, term, pack_ty))
    }
}
