use crate::{
    Parse, Rule,
    errors::{ParserError, UnknownKeyword},
};
use pest::iterators::Pair;
use syntax::types::{Bool, Type};

impl<Ty> Parse for Bool<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type ParseError = <Ty as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::const_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Bool<Ty>, Self::ParseError> {
        let bl = Bool::new();
        let p_str = p.as_str().trim().to_lowercase();
        if p_str == bl.to_string().to_lowercase() {
            Ok(bl)
        } else {
            Err(<UnknownKeyword as Into<ParserError>>::into(UnknownKeyword::new(&p_str)).into())
        }
    }
}
