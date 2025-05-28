use super::{pair_to_n_inner, pair_to_term, Error, Rule, Term};
use pest::iterators::Pair;
use syntax::terms::Pair as PairT;

pub fn pair_to_pair(p: Pair<'_, Rule>) -> Result<PairT<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Pair First", "Pair Second"])?;
    let fst_rule = inner.remove(0);
    let fst = pair_to_term(fst_rule)?;
    let snd_rule = inner.remove(0);
    let snd = pair_to_term(snd_rule)?;
    Ok(PairT::new(fst, snd))
}
