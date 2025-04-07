use super::{pair_to_type, Error, Rule};
use crate::syntax::types::RecordTy;
use pest::iterators::Pair;
use std::collections::HashMap;

pub fn pair_to_rec_ty(p: Pair<'_, Rule>) -> Result<RecordTy, Error> {
    let mut inner = p.into_inner();
    let mut records = HashMap::new();
    while let Some(var_rule) = inner.next() {
        let label = var_rule.as_str().trim().to_owned();
        let ty_rule = inner.next().ok_or(Error::missing("Record Type"))?;
        let ty = pair_to_type(ty_rule)?;
        records.insert(label, ty);
    }
    Ok(RecordTy { records })
}
