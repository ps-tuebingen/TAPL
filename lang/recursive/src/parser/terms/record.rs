use super::{pair_to_n_inner, pair_to_term, Error, Rule};
use crate::terms::{Record, RecordProj, Term};
use pest::iterators::Pair;
use std::collections::HashMap;

pub fn pair_to_record(p: Pair<'_, Rule>) -> Result<Record, Error> {
    let mut inner = p.into_inner();
    let mut recs = HashMap::new();
    while let Some(var_rule) = inner.next() {
        let var = var_rule.as_str().trim().to_owned();
        let term_rule = inner
            .next()
            .ok_or(Error::MissingInput("Record Term".to_owned()))?;
        let term = pair_to_term(term_rule)?;
        recs.insert(var, term);
    }
    Ok(Record { records: recs })
}

pub fn pair_to_proj(p: Pair<'_, Rule>, t: Term) -> Result<RecordProj, Error> {
    let label = pair_to_n_inner(p, vec!["Projection Target"])?
        .remove(0)
        .as_str()
        .trim()
        .to_owned();
    Ok(RecordProj {
        record: Box::new(t),
        label,
    })
}
