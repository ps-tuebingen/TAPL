use crate::{GroupParse, ParsableLanguage, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::terms::Right;

impl<Lang> Parse for Right<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::right_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Right<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Inr Argument", "Inr Type"])?;

        let arg_pair = inner.remove(0);
        let arg_term = Lang::Term::from_pair(
            pair_to_n_inner(arg_pair, vec!["Paren Term Inner"])?.remove(0),
            (),
        )?;

        let ty_pair = inner.remove(0);
        let ty = Lang::Type::from_pair(ty_pair, ())?;

        Ok(Right::new(arg_term, ty))
    }
}
