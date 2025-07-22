use crate::{Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::types::{Reference, Type};

impl<Ty> Parse for Reference<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();
    const RULE: Rule = Rule::ref_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Reference<Ty>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Ref Type"])?;
        let ty_rule = inner.remove(0);
        let ty = Ty::from_pair(ty_rule, ())?;
        Ok(Reference::new(ty))
    }
}
