use crate::{Parse, Rule, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::{
    terms::{Term, TyApp},
    types::Type,
};

impl<T, Ty> Parse for TyApp<T, Ty>
where
    T: Term + Parse,
    Ty: Type + Parse<LeftRecArg = ()>,
    <T as Parse>::ParseError: From<<Ty as Parse>::ParseError>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = T;

    const RULE: Rule = Rule::tyapp;

    fn from_pair(p: Pair<'_, Rule>, t: Self::LeftRecArg) -> Result<TyApp<T, Ty>, Self::ParseError> {
        let ty_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
        let ty = Ty::from_pair(ty_rule, ())?;
        Ok(TyApp::new(t, ty))
    }
}
