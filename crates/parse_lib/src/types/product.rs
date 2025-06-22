use crate::{Parse, Rule, errors::ParserError, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::types::{Product, Type};

impl<Ty> Parse for Product<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();
    const RULE: Rule = Rule::prod_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Product<Ty>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Pair First Type", "Pair Second Type"])?;
        let fst_rule = inner.remove(0);
        let fst = Ty::from_pair(fst_rule, ())?;
        let snd_rule = inner.remove(0);
        let snd = Ty::from_pair(snd_rule, ())?;
        Ok(Product::new(fst, snd))
    }
}
