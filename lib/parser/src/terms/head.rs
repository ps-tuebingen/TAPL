use crate::{GroupParse, ParsableLanguage, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::terms::Head;

impl<Lang> Parse for Head<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::head_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Head<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Head Type", "Head Argument"])?;

        let ty_rule = inner.remove(0);
        let ty = Lang::Type::from_pair(ty_rule, ())?;

        let term_pair = inner.remove(0);
        let term = Lang::Term::from_pair(term_pair, ())?;

        Ok(Head::new(term, ty))
    }
}
