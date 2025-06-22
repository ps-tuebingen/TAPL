use crate::{Parse, Rule, errors::ParserError, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::{
    terms::{Term, Unpack},
    types::Type,
};

impl<T, Ty> Parse for Unpack<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::unpack_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Unpack<T, Ty>, ParserError> {
        let mut inner = pair_to_n_inner(
            p,
            vec![
                "Unpack Type Name",
                "Unpack Term Name",
                "Pack Term",
                "Unpack Term",
            ],
        )?;
        let ty_name = inner.remove(0).as_str().trim();
        let term_name = inner.remove(0).as_str().trim();
        let pack_rule = inner.remove(0);
        let pack_term = T::from_pair(pack_rule, ())?;

        let unpack_rule = inner.remove(0);
        let unpack_term = T::from_pair(unpack_rule, ())?;
        Ok(Unpack::new(ty_name, term_name, pack_term, unpack_term))
    }
}
