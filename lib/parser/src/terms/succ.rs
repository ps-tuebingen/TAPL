use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::Succ};

impl<Lang> Parse for Succ<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::succ_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Succ<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Succ Argument"])?;
        let arg_rule = pair_to_n_inner(inner.remove(0), vec!["Prim Term Inner"])?.remove(0);
        let arg_term = Lang::Term::from_pair(arg_rule, ())?;
        Ok(Succ::new(arg_term))
    }
}
