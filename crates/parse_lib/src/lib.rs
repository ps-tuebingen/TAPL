pub mod errors;
mod kinds;
pub mod program;
pub mod sugar;
pub mod terms;
pub mod types;

use errors::{MissingInput, ParserError, RemainingInput};
use pest::{error::Error as PestErr, iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../../parse_lib/src/grammar.pest"]
pub struct LangParser;

pub trait Parse: Sized {
    type LeftRecArg;

    const RULE: Rule;

    fn from_pair(p: Pair<'_, Rule>, left_rec: Self::LeftRecArg) -> Result<Self, ParserError>;

    fn parse(source: String) -> Result<Self, ParserError>
    where
        Self::LeftRecArg: Default,
    {
        let mut pairs = LangParser::parse(Self::RULE, &source)
            .map_err(<PestErr<Rule> as Into<ParserError>>::into)?;
        let rule = pairs
            .next()
            .ok_or(<MissingInput as Into<ParserError>>::into(
                MissingInput::new(&format!("{:?}", Self::RULE)),
            ))?;
        let result = Self::from_pair(rule, Default::default())?;
        if let Some(rule) = pairs.next() {
            Err(
                <RemainingInput as Into<ParserError>>::into(RemainingInput::new(&format!(
                    "{:?}",
                    rule
                )))
                .into(),
            )
        } else {
            Ok(result)
        }
    }
}

pub trait GroupParse: Sized {
    type ParseError: std::error::Error + From<ParserError>;

    const RULE: Rule;

    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Self, ParserError>;
    fn from_pair_leftrec(p: Pair<'_, Rule>, left_rec: Self) -> Result<Self, ParserError>;
}

impl<T> Parse for T
where
    T: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = <T as GroupParse>::RULE;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = p.into_inner();
        let prim_rule = inner
            .next()
            .ok_or(<MissingInput as Into<ParserError>>::into(
                MissingInput::new("Non Left-Recursive"),
            ))?;
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
            return Err(
                <RemainingInput as Into<ParserError>>::into(RemainingInput::new(&format!("{n:?}")))
                    .into(),
            );
        }
        Ok(slf)
    }
}

pub fn pair_to_n_inner<'a>(
    p: Pair<'a, Rule>,
    names: Vec<&str>,
) -> Result<Vec<Pair<'a, Rule>>, ParserError> {
    let mut inner = p.into_inner();
    let mut pairs = vec![];
    for name in names {
        let next = inner.next().ok_or(MissingInput::new(name))?;
        pairs.push(next)
    }
    if let Some(n) = inner.next() {
        return Err(RemainingInput::new(&format!("{n:?}")).into());
    }
    Ok(pairs)
}
