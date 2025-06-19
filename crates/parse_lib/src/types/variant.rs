use crate::{
    errors::{MissingInput, ParserError},
    Parse, Rule,
};
use pest::iterators::Pair;
use std::collections::HashMap;
use syntax::types::{Type, Variant};

impl<Ty> Parse for Variant<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type ParseError = <Ty as Parse>::ParseError;
    type LeftRecArg = ();
    const RULE: Rule = Rule::variant_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Variant<Ty>, Self::ParseError> {
        let mut inner = p.into_inner();
        let mut variants = HashMap::new();
        while let Some(label_rule) = inner.next() {
            let label = label_rule.as_str().trim().to_owned();
            let ty_rule = inner
                .next()
                .ok_or(<MissingInput as Into<ParserError>>::into(
                    MissingInput::new("Variant Type"),
                ))?;
            let ty = Ty::from_pair(ty_rule, ())?;
            variants.insert(label, ty);
        }
        Ok(Variant::new(variants))
    }
}
