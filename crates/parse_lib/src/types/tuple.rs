use crate::{Parse, Rule, errors::ParserError};
use pest::iterators::Pair;
use syntax::types::{Tuple, Type};

impl<Ty> Parse for Tuple<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::tuple_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Tuple<Ty>, ParserError> {
        let mut tys = vec![];
        for inner_pair in p.into_inner() {
            let inner_ty = Ty::from_pair(inner_pair, ())?;
            tys.push(inner_ty)
        }
        Ok(Tuple::new(tys))
    }
}
