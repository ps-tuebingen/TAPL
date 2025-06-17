use crate::{Parse, Rule};
use pest::iterators::Pair;
use syntax::types::{Type, TypeVariable};

impl<Ty> Parse for TypeVariable<Ty>
where
    Ty: Type + Parse,
{
    type ParseError = <Ty as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::variable;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Self, Self::ParseError> {
        Ok(TypeVariable::new(p.as_str().trim()))
    }
}
