use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::Variant};

impl<Lang> Parse for Variant<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::variant_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Variant<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Variant Label", "Variant Term", "Variant Type"])?;

        let var_pair = inner.remove(0);
        let var = var_pair.as_str();

        let term_pair = inner.remove(0);
        let variant_term = Lang::Term::from_pair(term_pair, ())?;

        let ty_pair = inner.remove(0);
        let variant_ty = Lang::Type::from_pair(ty_pair, ())?;

        Ok(Variant::new(var, variant_term, variant_ty))
    }
}
