use super::pair_to_term;
use crate::{
    parser::{errors::Error, get_n_inner, Rule},
    syntax::{Proj, RecordProj, Term},
};
use pest::iterators::Pair;

pub fn pair_to_proj(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = get_n_inner(p, vec!["Projection Term", "Projection Target"])?;

    let proj_pair = inner.remove(0);
    let proj_term = pair_to_term(proj_pair)?;

    let target_pair = inner.remove(0);
    let target_pair_str = target_pair.as_str().trim();
    let term = match target_pair_str.parse::<usize>() {
        Ok(num) => Proj {
            tup: Box::new(proj_term),
            ind: num,
        }
        .into(),
        Err(_) => RecordProj {
            record: Box::new(proj_term),
            label: target_pair_str.to_owned(),
        }
        .into(),
    };
    Ok(term)
}
