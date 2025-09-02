use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, types::Product};

impl<Lang> Parse for Product<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();
    const RULE: Rule = Rule::prod_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Product<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Pair First Type", "Pair Second Type"])?;
        let fst_rule = inner.remove(0);
        let fst = Lang::Type::from_pair(fst_rule, ())?;
        let snd_rule = inner.remove(0);
        let snd = Lang::Type::from_pair(snd_rule, ())?;
        Ok(Product::new(fst, snd))
    }
}
