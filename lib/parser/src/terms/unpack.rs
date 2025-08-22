use crate::{GroupParse, ParsableLanguage, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::terms::Unpack;

impl<Lang> Parse for Unpack<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::unpack_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Unpack<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(
            p,
            vec![
                "Unpack Type Name",
                "Unpack Term Name",
                "Pack Term",
                "Unpack Term",
            ],
        )?;
        let ty_name = inner.remove(0).as_str().trim();
        let term_name = inner.remove(0).as_str().trim();
        let pack_rule = inner.remove(0);
        let pack_term = Lang::Term::from_pair(pack_rule, ())?;

        let unpack_rule = inner.remove(0);
        let unpack_term = Lang::Term::from_pair(unpack_rule, ())?;
        Ok(Unpack::new(ty_name, term_name, pack_term, unpack_term))
    }
}
