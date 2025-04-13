use super::{pair_to_term, to_parse_err};
use crate::{parser::Rule, syntax::Record};
use common::errors::{Error, ErrorKind};
use pest::iterators::Pair;
use std::collections::HashMap;

pub fn pair_to_rec(p: Pair<'_, Rule>) -> Result<Record, Error> {
    let mut records = HashMap::new();

    let mut inner = p.into_inner();
    while let Some(n) = inner.next() {
        let var = n.as_str().trim().to_owned();
        let next_pair = inner.next().ok_or(to_parse_err(ErrorKind::MissingInput(
            "Record Term".to_owned(),
        )))?;
        let n_term = pair_to_term(next_pair)?;
        records.insert(var, n_term);
    }
    Ok(Record { records })
}
