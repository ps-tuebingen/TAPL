use crate::{Parse, Rule};
use errors::{UnknownKeyword, parse_error::ParserError};
use pest::iterators::Pair;
use syntax::types::{Bool, Type};

impl<Ty> Parse for Bool<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::const_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Bool<Ty>, ParserError> {
        let bl = Bool::new();
        let p_str = p.as_str().trim().to_lowercase();
        if p_str == bl.to_string().to_lowercase() {
            Ok(bl)
        } else {
            Err(UnknownKeyword::new(&p_str).into())
        }
    }
}
