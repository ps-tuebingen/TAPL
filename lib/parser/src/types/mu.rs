use crate::{GroupParse, ParsableLanguage, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::types::Mu;

impl<Lang> Parse for Mu<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::mu_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Mu<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Mu Variable", "Mu Body"])?;
        let var = inner.remove(0).as_str().trim();
        let ty_rule = inner.remove(0);
        let ty = Lang::Type::from_pair(ty_rule, ())?;
        Ok(Mu::new(var, ty))
    }
}
