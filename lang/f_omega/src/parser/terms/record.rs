use super::{pair_to_term, to_parse_err, Rule};
use crate::syntax::terms::Record;
use common::errors::{Error, ErrorKind};
use pest::iterators::Pair;
use std::collections::HashMap;

pub fn pair_to_record(p: Pair<'_, Rule>) -> Result<Record, Error> {
    let mut inner = p.into_inner();
    let mut records = HashMap::new();
    while let Some(var_rule) = inner.next() {
        let label = var_rule.as_str().trim().to_owned();
        let term_rule = inner.next().ok_or(to_parse_err(ErrorKind::MissingInput(
            "Record Term".to_owned(),
        )))?;
        let term = pair_to_term(term_rule)?;
        records.insert(label, term);
    }
    Ok(Record { records })
}
