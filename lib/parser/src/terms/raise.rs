use crate::{GroupParse, ParsableLanguage, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::terms::Raise;

impl<Lang> Parse for Raise<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::raise_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Raise<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(
            p,
            vec![
                "Raise Continuation Type",
                "Raise Exception Type",
                "Raise Term",
            ],
        )?;
        let cont_ty_rule = inner.remove(0);
        let cont_ty = Lang::Type::from_pair(cont_ty_rule, ())?;
        let ex_ty_rule = inner.remove(0);
        let ex_ty = Lang::Type::from_pair(ex_ty_rule, ())?;
        let catch_rule = inner.remove(0);
        let catch_term = Lang::Term::from_pair(catch_rule, ())?;
        Ok(Raise::new(catch_term, cont_ty, ex_ty))
    }
}
