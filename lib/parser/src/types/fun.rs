use crate::{GroupParse, ParsableLanguage, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::types::Fun;

impl<Lang> Parse for Fun<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = Lang::Type;

    const RULE: Rule = Rule::fun_type;
    fn from_pair(p: Pair<'_, Rule>, ty: Self::LeftRecArg) -> Result<Fun<Lang>, ParserError> {
        let to_rule = pair_to_n_inner(p, vec!["Function Return Type"])?.remove(0);
        let to_ty = Lang::Type::from_pair(to_rule, ())?;
        Ok(Fun::new(ty, to_ty))
    }
}
