use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::{MissingInput, parse_error::ParserError};
use pest::iterators::Pair;
use syntax::{
    language::Language,
    terms::{VariantCase, variantcase::VariantPattern},
};

impl<Lang> Parse for VariantCase<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::variantcase_term;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = p.into_inner();
        let bound_rule = inner.next().ok_or_else(|| {
            <MissingInput as Into<ParserError>>::into(MissingInput::new("Case Bound Term"))
        })?;
        let bound_term = Lang::Term::from_pair(bound_rule, ())?;
        let mut patterns = vec![];
        for pattern_rule in inner {
            patterns.push(VariantPattern::<Lang>::from_pair(pattern_rule, ())?);
        }
        Ok(Self::new(bound_term, patterns))
    }
}

impl<Lang> Parse for VariantPattern<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::variant_pattern;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
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
        let term = Lang::Term::from_pair(term_rule, ())?;
        Ok(Self::new(label, var, term))
    }
}
