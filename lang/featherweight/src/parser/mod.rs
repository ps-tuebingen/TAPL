use crate::syntax::ClassTable;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

mod class;
mod constructor;
pub mod errors;
mod methods;
mod terms;
use class::pair_to_classdecl;
use errors::Error;

#[derive(Parser)]
#[grammar = "parser/featherweight.pest"]
struct FeatherweightParser;

pub fn parse(input: String) -> Result<ClassTable, Error> {
    let mut parsed = FeatherweightParser::parse(Rule::program, &input)?;
    let prog_rule = parsed
        .next()
        .ok_or(Error::MissingInput("Program".to_owned()))?;
    if let Some(n) = parsed.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }

    let mut classes = ClassTable::default();
    for inner_rule in prog_rule.into_inner() {
        if inner_rule.as_rule() == Rule::EOI {
            break;
        }
        let class = pair_to_classdecl(inner_rule)?;
        classes.classes.insert(class.name.clone(), class);
    }
    Ok(classes)
}

pub fn pair_to_n_inner<'a>(
    p: Pair<'a, Rule>,
    names: Vec<&str>,
) -> Result<Vec<Pair<'a, Rule>>, Error> {
    let mut pairs = vec![];
    let mut inner = p.into_inner();
    for name in names {
        let next = inner.next().ok_or(Error::MissingInput(name.to_owned()))?;
        pairs.push(next);
    }

    if let Some(n) = inner.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }
    Ok(pairs)
}
