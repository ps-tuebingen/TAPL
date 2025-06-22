use crate::{
    errors::{ParserError, UnknownKeyword},
    Parse, Rule,
};
use pest::iterators::Pair;
use syntax::types::{Nat, Type};

impl<Ty> Parse for Nat<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::const_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Nat<Ty>, ParserError> {
        let nat = Nat::new();
        let p_str = p.as_str().trim().to_lowercase();
        if p_str == nat.to_string().to_lowercase() {
            Ok(nat)
        } else {
            Err(<UnknownKeyword as Into<ParserError>>::into(UnknownKeyword::new(&p_str)).into())
        }
    }
}
