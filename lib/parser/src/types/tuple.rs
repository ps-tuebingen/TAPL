use crate::{GroupParse, Parse, Rule};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, types::Tuple};

impl<Lang> Parse for Tuple<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::tuple_type;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut tys = vec![];
        for inner_pair in p.into_inner() {
            let inner_ty = Lang::Type::from_pair(inner_pair, ())?;
            tys.push(inner_ty);
        }
        Ok(Self::new(tys))
    }
}
