use crate::{GroupParse, Parse, Rule};
use errors::{MissingInput, RemainingInput, parse_error::ParserError};
use pest::iterators::Pair;
use syntax::{language::Language, terms::Tail};

impl<Lang> Parse for Tail<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::tail_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Tail<Lang>, ParserError> {
        let mut inner = p.into_inner();
        let ty_rule = inner.next().ok_or(MissingInput::new("Head Type"))?;
        let ty = Lang::Type::from_pair(ty_rule, ())?;

        let term_pair = inner.next().ok_or(MissingInput::new("Head Argument"))?;
        let term = Lang::Term::from_pair(term_pair, ())?;

        if let Some(next) = inner.next() {
            return Err(RemainingInput::new(&format!("{:?}", next.as_rule())).into());
        }
        Ok(Tail::new(term, ty))
    }
}
