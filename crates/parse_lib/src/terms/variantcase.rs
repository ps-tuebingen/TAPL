use crate::{
    errors::{MissingInput, ParserError},
    pair_to_n_inner, Parse, Rule,
};
use pest::iterators::Pair;
use syntax::terms::{variantcase::VariantPattern, Term, VariantCase};

impl<T> Parse for VariantCase<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::variantcase_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<VariantCase<T>, ParserError> {
        let mut inner = p.into_inner();
        let bound_rule = inner
            .next()
            .ok_or(<MissingInput as Into<ParserError>>::into(
                MissingInput::new("Case Bound Term"),
            ))?;
        let bound_term = T::from_pair(bound_rule, ())?;
        let mut patterns = vec![];
        for pattern_rule in inner {
            patterns.push(VariantPattern::<T>::from_pair(pattern_rule, ())?);
        }
        Ok(VariantCase::new(bound_term, patterns))
    }
}

impl<T> Parse for VariantPattern<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::variant_pattern;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<VariantPattern<T>, ParserError> {
        let mut inner = pair_to_n_inner(
            p,
            vec![
                "Variant Pattern Label",
                "Variant Pattern Var",
                "Variant Pattern Right-Hand Side",
            ],
        )?;
        let label = inner.remove(0).as_str().trim();
        let var = inner.remove(0).as_str().trim();
        let term_rule = inner.remove(0);
        let term = T::from_pair(term_rule, ())?;
        Ok(VariantPattern::new(label, var, term))
    }
}
