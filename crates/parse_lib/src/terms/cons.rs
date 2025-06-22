use crate::{Parse, Rule, errors::ParserError, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::{
    terms::{Cons, Term},
    types::Type,
};

impl<T, Ty> Parse for Cons<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::cons_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Cons<T, Ty>, ParserError> {
        let mut inner = pair_to_n_inner(
            p,
            vec!["Cons Type", "First Const Argument", "Second Cons Argument"],
        )?;

        let ty_pair = inner.remove(0);
        let ty = Ty::from_pair(ty_pair, ())?;

        let fst_pair = inner.remove(0);
        let fst = T::from_pair(fst_pair, ())?;

        let snd_pair = inner.remove(0);
        let snd = T::from_pair(snd_pair, ())?;

        Ok(Cons::new(fst, snd, ty))
    }
}
