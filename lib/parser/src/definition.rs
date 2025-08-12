use crate::{Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{definition::Definition, terms::Term, types::Type};

impl<T, Ty> Parse for Definition<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::top_level_def;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = pair_to_n_inner(
            p,
            vec!["Definition Name", "Definition Annot", "Definition Body"],
        )?;
        let name = inner.remove(0).as_str().trim();
        let annot_rule = inner.remove(0);
        let annot = Ty::from_pair(annot_rule, ())?;
        let body_rule = inner.remove(0);
        let body = T::from_pair(body_rule, ())?;
        Ok(Definition::new(name, annot, body))
    }
}
