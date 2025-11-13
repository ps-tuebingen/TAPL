pub mod definition;
mod kinds;
pub mod program;
pub mod sugar;
pub mod terms;
pub mod types;
mod untyped;

use errors::{MissingInput, RemainingInput, parse_error::ParserError};
use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../../parser/src/grammar.pest"]
pub struct LangParser;

/// Trait for parsing arbitrary languag constructs
pub trait Parse: Sized {
    /// For left-recursive `Self`, the type that needs to be provided
    /// for example, for [`syntax::terms::app::App`] this is `Self::Lang::Term`
    /// and for [`syntax::terms::tyapp::TyApp`] this is `Self::Lang::Type`
    /// if `Self` is not left-recursive, this is `()`
    type LeftRecArg;

    /// The rule corresponding to `Self`
    const RULE: Rule;

    /// parse `Self` from a pair with given left value, if this is left-recursive
    /// # Errors
    /// Returns an error if the pair does not correspond to a valid `Self`
    fn from_pair(p: Pair<'_, Rule>, left_rec: Self::LeftRecArg) -> Result<Self, ParserError>;

    /// Parse a string input to a given `Self`
    /// in this case `Self` is usually a [`syntax::language::Language`] Term or Type which calls
    /// [`Parse::from_pair`] on each possible term/type
    /// # Errors
    /// Returns an error if the input can not be parsed by pest, or the resulting rules are
    /// malformed
    fn parse(source: String) -> Result<Self, ParserError>
    where
        Self::LeftRecArg: Default,
    {
        let mut pairs = LangParser::parse(Self::RULE, &source)?;
        let rule = pairs
            .next()
            .ok_or_else(|| MissingInput::new(&format!("{:?}", Self::RULE)))?;
        let result = Self::from_pair(rule, Default::default())?;
        pairs.next().map_or_else(
            || Ok(result),
            |rule| Err(RemainingInput::new(&format!("{rule:?}",)).into()),
        )
    }
}

/// trait for parsing a group of terms/types
/// used for the language enums
pub trait GroupParse: Sized {
    /// Rule corresponding to `Self`
    /// usually [`Rule::term`] or [`Rule::type`]
    const RULE: Rule;

    /// Parse `Self` from a pair without left recursion
    /// used for terms like `\x:T.t`
    /// # Errors
    /// Returns an error if the generated pair does not correspond to a valid `Self`
    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Self, ParserError>;

    /// Parse `Self` from a left-recursive pair with the first argument already given
    /// used for terms like `t1 t2`
    /// # Errors
    /// Returns an error if the generated pair does not correspond to a valid `Self`
    fn from_pair_leftrec(p: Pair<'_, Rule>, left_rec: Self) -> Result<Self, ParserError>;
}

impl<T> Parse for T
where
    T: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = <T as GroupParse>::RULE;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = p.into_inner();
        let prim_rule = inner.next().ok_or_else(|| {
            <MissingInput as Into<ParserError>>::into(MissingInput::new("Non Left-Recursive"))
        })?;
        let prim_inner = pair_to_n_inner(prim_rule, vec!["Non Left-Recursive"])?.remove(0);
        let prim_self = Self::from_pair_nonrec(prim_inner)?;

        let slf = match inner.next() {
            None => prim_self,
            Some(leftrec) => {
                let leftrec_inner =
                    pair_to_n_inner(leftrec, vec!["Left Recursive Term"])?.remove(0);
                Self::from_pair_leftrec(leftrec_inner, prim_self)?
            }
        };

        if let Some(n) = inner.next() {
            return Err(RemainingInput::new(&format!("{n:?}")).into());
        }
        Ok(slf)
    }
}

/// Given a parser pair and expected inner rules, extracts these inner rules
/// # Errors
/// Returns an error if the number of inner rules does not match the number of given rules
/// The names in the argument are only used for errors
pub fn pair_to_n_inner<'a>(
    p: Pair<'a, Rule>,
    names: Vec<&str>,
) -> Result<Vec<Pair<'a, Rule>>, ParserError> {
    let mut inner = p.into_inner();
    let mut pairs = vec![];
    for name in names {
        let next = inner.next().ok_or_else(|| MissingInput::new(name))?;
        pairs.push(next);
    }
    if let Some(n) = inner.next() {
        return Err(RemainingInput::new(&format!("{n:?}")).into());
    }
    Ok(pairs)
}
