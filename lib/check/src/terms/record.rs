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
        let features = Lang::features();
        let mut premises = vec![];

        let mut recs = HashMap::new();
        let mut rec_knd = None;
        for (lb, t) in self.records.iter() {
            let term_res = t.check(env.clone())?;
            let ty = term_res.ret_ty();
            premises.push(term_res);

            let ty_norm;
            if features.normalizing {
                let ty_norm_deriv = ty.normalize(env.clone());
                ty_norm = ty_norm_deriv.ret_ty();
                premises.push(ty_norm_deriv);
            } else {
                ty_norm = ty;
            }

            if features.kinded {
                let ty_knd = ty_norm.check_kind(env.clone())?;
                match rec_knd {
                    None => {
                        rec_knd = Some(ty_knd);
                    }
                    Some(ref knd) => {
                        knd.check_equal(&ty_knd)?;
                    }
                }
            }

            recs.insert(lb.clone(), ty_norm);
        }

        let conc = TypingConclusion::new(env, self.clone(), RecordTy::<Lang>::new(recs));
        let deriv = TypingDerivation::record(conc, premises);
        Ok(deriv.into())
    }
}
