use crate::{GroupParse, ParsableLanguage, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::terms::Fix;

impl<Lang> Parse for Fix<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::fix_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Fix<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Fix Term"])?;
        let inner_rule = pair_to_n_inner(inner.remove(0), vec!["Prim Term Inner"])?.remove(0);
        let inner = Lang::Term::from_pair(inner_rule, ())?;
        Ok(Fix::new(inner))
    }
}
