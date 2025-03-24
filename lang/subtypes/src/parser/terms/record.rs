use super::pair_to_term;
use crate::{
    parser::{errors::Error, pair_to_n_inner, Rule},
    syntax::{Projection, Record, Term},
};
use pest::iterators::Pair;
use std::collections::HashMap;

pub fn pair_to_record(p: Pair<'_, Rule>) -> Result<Record, Error> {
    let mut inner = p.into_inner();
    let mut records = HashMap::new();
    while let Some(next) = inner.next() {
        let var = next.as_str().trim().to_owned();
        let term_rule = inner
            .next()
            .ok_or(Error::MissingInput("Record Term".to_owned()))?;
        let term = pair_to_term(term_rule)?;
        records.insert(var, term);
    }
    Ok(Record { records })
}

pub fn pair_to_proj(p: Pair<'_, Rule>, t: Term) -> Result<Projection, Error> {
    let label = pair_to_n_inner(p, vec!["Projection Target"])?
        .remove(0)
        .as_str()
        .trim()
        .to_owned();
    Ok(Projection {
        record: Box::new(t),
        label,
    })
}
