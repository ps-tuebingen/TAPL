use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, types::Sink};

impl<Lang> Parse for Sink<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::sink_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Sink<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Sink Type"])?;
        let ty_rule = inner.remove(0);
        let ty = Lang::Type::from_pair(ty_rule, ())?;
        Ok(Sink::new(ty))
    }
}
