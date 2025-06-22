use crate::{Parse, Rule, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::{
    terms::{Term, Variant},
    types::Type,
};

impl<T, Ty> Parse for Variant<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
    <T as Parse>::ParseError: From<<Ty as Parse>::ParseError>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::variant_term;

    fn from_pair(
        p: Pair<'_, Rule>,
        _: Self::LeftRecArg,
    ) -> Result<Variant<T, Ty>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Variant Label", "Variant Term", "Variant Type"])?;

        let var_pair = inner.remove(0);
        let var = var_pair.as_str();

        let term_pair = inner.remove(0);
        let variant_term = T::from_pair(term_pair, ())?;

        let ty_pair = inner.remove(0);
        let variant_ty = Ty::from_pair(ty_pair, ())?;

        Ok(Variant::new(var, variant_term, variant_ty))
    }
}
