use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use std::collections::HashMap;
use syntax::{env::Environment, language::Language, terms::Record, types::Record as RecordTy};

impl<Lang> Typecheck for Record<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type: Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    RecordTy<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let mut recs = HashMap::new();
        let mut ress = Vec::new();
        let mut rec_knd = None;
        for (lb, t) in self.records.iter() {
            let term_res = t.check(env.clone())?;
            let ty = term_res.ret_ty().normalize(env.clone());
            ress.push(term_res);

            let ty_knd = ty.check_kind(env.clone())?;
            recs.insert(lb.clone(), ty);
            match rec_knd {
                None => {
                    rec_knd = Some(ty_knd);
                }
                Some(ref knd) => {
                    knd.check_equal(&ty_knd)?;
                }
            }
        }

        let conc = TypingConclusion::new(env, self.clone(), RecordTy::<Lang>::new(recs));
        let deriv = TypingDerivation::record(conc, ress);
        Ok(deriv.into())
    }
}
