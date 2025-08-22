use crate::{GroupParse, ParsableLanguage, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{kinds::Kind, types::Exists};

impl<Lang> Parse for Exists<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::exists_kinded_type;
    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Exists<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Exists Variable", "Exists kind", "Exists Type"])?;
        let start_rule = inner.remove(0);
        let mut start_inner = pair_to_n_inner(start_rule, vec!["Exists Variable"])?;
        let var = start_inner.remove(0).as_str().trim();

        let kind_rule = inner.remove(0);
        let kind = Kind::from_pair(kind_rule, ())?;
        let ty_rule = inner.remove(0);
        let inner_ty = Lang::Type::from_pair(ty_rule, ())?;
        Ok(Exists::new(var, kind, inner_ty))
    }
}
