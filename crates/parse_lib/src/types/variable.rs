use crate::{errors::ParserError, Parse, Rule};
use pest::iterators::Pair;
use syntax::types::{Type, TypeVariable};

impl<Ty> Parse for TypeVariable<Ty>
where
    Ty: Type + Parse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::type_variable;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Self, ParserError> {
        Ok(TypeVariable::new(p.as_str().trim()))
    }
}
