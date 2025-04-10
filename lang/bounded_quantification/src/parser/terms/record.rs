use super::{pair_to_n_inner, pair_to_term, to_parse_err, Error, Rule};
use crate::syntax::{Projection, Record, Term};
use common::errors::ErrorKind;
use pest::iterators::Pair;
use std::collections::HashMap;

pub fn pair_to_rec(p: Pair<'_, Rule>) -> Result<Record, Error> {
    let mut inner = p.into_inner();
    let mut recs = HashMap::new();
    while let Some(label_rule) = inner.next() {
        let label = label_rule.as_str().trim().to_owned();
        let term_rule = inner
            .next()
            .ok_or(ErrorKind::MissingInput("Record Term".to_owned()))
            .map_err(to_parse_err)?;
        let term = pair_to_term(term_rule)?;
        recs.insert(label, term);
    }
    Ok(Record { records: recs })
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
