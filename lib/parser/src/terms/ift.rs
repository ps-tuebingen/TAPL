use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::If};

impl<Lang> Parse for If<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::if_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<If<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["If Condition", "Then Term", "Else Term"])?;

        let ift_rule = inner.remove(0);
        let ift = Lang::Term::from_pair(ift_rule, ())?;
        let thent_rule = inner.remove(0);
        let thent = Lang::Term::from_pair(thent_rule, ())?;
        let elset_rule = inner.remove(0);
        let elset = Lang::Term::from_pair(elset_rule, ())?;
        Ok(If::new(ift, thent, elset))
    }
}
