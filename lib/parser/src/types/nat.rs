use crate::{Parse, Rule};
use errors::{UnknownKeyword, parse_error::ParserError};
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
            Err(UnknownKeyword::new(&p_str).into())
        }
    }
}
