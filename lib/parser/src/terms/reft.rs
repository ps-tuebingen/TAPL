use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::Ref};

impl<Lang> Parse for Ref<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::ref_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Ref<Lang>, ParserError> {
        let mut inner_rules = pair_to_n_inner(p, vec!["Ref Term"])?;
        let term_rule = inner_rules.remove(0);
        let term = Lang::Term::from_pair(
            pair_to_n_inner(term_rule, vec!["Ref Argument"])?.remove(0),
            (),
        )?;
        Ok(Ref::new(term))
    }
}
