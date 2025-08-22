use crate::{GroupParse, ParsableLanguage, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::types::OpApp;

impl<Lang> Parse for OpApp<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = Lang::Type;

    const RULE: Rule = Rule::op_app_type;

    fn from_pair(p: Pair<'_, Rule>, ty: Self::LeftRecArg) -> Result<OpApp<Lang>, ParserError> {
        let arg_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
        let arg = Lang::Type::from_pair(arg_rule, ())?;
        Ok(OpApp::new(ty, arg))
    }
}
