use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::types::{Sum, Type};

impl<Ty> Parse for Sum<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type ParseError = <Ty as Parse>::ParseError;
    type LeftRecArg = ();
    const RULE: Rule = Rule::sum_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Sum<Ty>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["First Sum Type", "Second Sum Type"])?;

        let fst_pair = inner.remove(0);
        let fst_ty = Ty::from_pair(fst_pair, ())?;

        let snd_pair = inner.remove(0);
        let snd_ty = Ty::from_pair(snd_pair, ())?;

        Ok(Sum::new(fst_ty, snd_ty))
    }
}
