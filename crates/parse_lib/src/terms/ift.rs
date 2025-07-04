use crate::{Parse, Rule, errors::ParserError, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::terms::{If, Term};

impl<T> Parse for If<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::if_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<If<T>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["If Condition", "Then Term", "Else Term"])?;

        let ift_rule = inner.remove(0);
        let ift = T::from_pair(ift_rule, ())?;
        let thent_rule = inner.remove(0);
        let thent = T::from_pair(thent_rule, ())?;
        let elset_rule = inner.remove(0);
        let elset = T::from_pair(elset_rule, ())?;
        Ok(If::new(ift, thent, elset))
    }
}
