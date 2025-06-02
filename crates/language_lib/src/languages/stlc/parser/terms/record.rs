use super::{pair_to_term, Error, MissingInput, Rule, Term};
use pest::iterators::Pair;
use std::collections::HashMap;
use syntax::terms::Record;

pub fn pair_to_rec(p: Pair<'_, Rule>) -> Result<Record<Term>, Error> {
    let mut records = HashMap::new();

    let mut inner = p.into_inner();
    while let Some(n) = inner.next() {
        let var = n.as_str().trim().to_owned();
        let next_pair = inner.next().ok_or(MissingInput::new("Record Term"))?;
        let n_term = pair_to_term(next_pair)?;
        records.insert(var, n_term);
    }
    Ok(Record::new(records))
}
