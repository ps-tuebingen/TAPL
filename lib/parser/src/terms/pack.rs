use crate::{GroupParse, ParsableLanguage, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::terms::Pack;

impl<Lang> Parse for Pack<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::pack_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Pack<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Packed Type", "Packed Term", "Pack Type"])?;
        let packed_rule = inner.remove(0);
        let packed_ty = Lang::Type::from_pair(packed_rule, ())?;

        let term_rule = inner.remove(0);
        let term = Lang::Term::from_pair(term_rule, ())?;

        let pack_rule = inner.remove(0);
        let pack_ty = Lang::Type::from_pair(pack_rule, ())?;

        Ok(Pack::new(packed_ty, term, pack_ty))
    }
}
