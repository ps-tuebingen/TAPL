use crate::{GroupParse, ParsableLanguage, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::terms::Try;

impl<Lang> Parse for Try<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::try_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Try<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Try Term", "With Term"])?;

        let tryt_rule = inner.remove(0);
        let try_t = Lang::Term::from_pair(tryt_rule, ())?;
        let with_rule = inner.remove(0);
        let witht = Lang::Term::from_pair(with_rule, ())?;

        Ok(Try::new(try_t, witht))
    }
}
