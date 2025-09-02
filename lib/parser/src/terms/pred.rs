use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::Pred};

impl<Lang> Parse for Pred<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::pred_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Pred<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Pred Argument"])?;
        let arg_rule = pair_to_n_inner(inner.remove(0), vec!["Prim Term Inner"])?.remove(0);
        let arg = Lang::Term::from_pair(arg_rule, ())?;
        Ok(Pred::new(arg))
    }
}
