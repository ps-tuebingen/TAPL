use super::{pair_to_type, to_parse_err, Error, Rule};
use crate::types::Type;
use common::{errors::ErrorKind, types::Record};
use pest::iterators::Pair;
use std::collections::HashMap;

pub fn pair_to_rec_ty(p: Pair<'_, Rule>) -> Result<Record<Type>, Error> {
    let mut inner = p.into_inner();
    let mut records = HashMap::new();
    while let Some(var_rule) = inner.next() {
        let label = var_rule.as_str().trim().to_owned();
        let ty_rule = inner.next().ok_or(to_parse_err(ErrorKind::MissingInput(
            "Record Type".to_owned(),
        )))?;
        let ty = pair_to_type(ty_rule)?;
        records.insert(label, ty);
    }
    Ok(Record::new(records))
}
