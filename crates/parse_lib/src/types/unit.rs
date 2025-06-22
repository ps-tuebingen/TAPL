use crate::{
    errors::{ParserError, UnknownKeyword},
    Parse, Rule,
};
use pest::iterators::Pair;
use syntax::types::{Type, Unit};

impl<Ty> Parse for Unit<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::const_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Unit<Ty>, ParserError> {
        let u = Unit::new();
        let p_str = p.as_str().trim().to_lowercase();
        if p_str == u.to_string().to_lowercase() {
            Ok(u)
        } else {
            Err(<UnknownKeyword as Into<ParserError>>::into(UnknownKeyword::new(&p_str)).into())
        }
    }
}
