use super::{pair_to_n_inner, pair_to_term, Error, MissingInput, Rule, Term};
use pest::iterators::Pair;
use std::collections::HashMap;
use syntax::terms::{Record, RecordProj};

pub fn pair_to_record(p: Pair<'_, Rule>) -> Result<Record<Term>, Error> {
    let mut inner = p.into_inner();
    let mut recs = HashMap::new();
    while let Some(var_rule) = inner.next() {
        let var = var_rule.as_str().trim().to_owned();
        let term_rule = inner.next().ok_or(MissingInput::new("Record Term"))?;
        let term = pair_to_term(term_rule)?;
        recs.insert(var, term);
    }
    Ok(Record::new(recs))
}

pub fn pair_to_proj(p: Pair<'_, Rule>, t: Term) -> Result<RecordProj<Term>, Error> {
    let label = pair_to_n_inner(p, vec!["Projection Target"])?
        .remove(0)
        .as_str()
        .trim();
    Ok(RecordProj::new(t, label))
}
