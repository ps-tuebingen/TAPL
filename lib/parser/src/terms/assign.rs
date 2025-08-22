use crate::{GroupParse, ParsableLanguage, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::terms::Assign;

impl<Lang> Parse for Assign<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = Lang::Term;

    const RULE: Rule = Rule::assign;

    fn from_pair(p: Pair<'_, Rule>, t: Self::LeftRecArg) -> Result<Assign<Lang>, ParserError> {
        let term_rule = pair_to_n_inner(p, vec!["Assign Right hand side"])?.remove(0);
        let rhs = Lang::Term::from_pair(term_rule, ())?;
        Ok(Assign::new(t, rhs))
    }
}
