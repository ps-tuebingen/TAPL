use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::Left};

impl<Lang> Parse for Left<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::left_term;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Inl Argument", "Inl Type"])?;

        let arg_pair = inner.remove(0);
        let arg_term = Lang::Term::from_pair(
            pair_to_n_inner(arg_pair, vec!["Paren Term Inner"])?.remove(0),
            (),
        )?;

        let ty_pair = inner.remove(0);
        let ty = Lang::Type::from_pair(ty_pair, ())?;

        Ok(Self::new(arg_term, ty))
    }
}
