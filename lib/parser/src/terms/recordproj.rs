use crate::{GroupParse, ParsableLanguage, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::terms::RecordProj;

impl<Lang> Parse for RecordProj<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = Lang::Term;

    const RULE: Rule = Rule::record_proj;

    fn from_pair(p: Pair<'_, Rule>, t: Self::LeftRecArg) -> Result<RecordProj<Lang>, ParserError> {
        let label = pair_to_n_inner(p, vec!["Projection Target"])?
            .remove(0)
            .as_str()
            .trim();
        Ok(RecordProj::new(t, label))
    }
}
