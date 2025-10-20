use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::Tuple, types::Tuple as TupleTy};

impl<Lang> Typecheck for Tuple<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type: Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    TupleTy<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let mut tys: Vec<<Self::Lang as Language>::Type> = vec![];
        let mut knd = None;

        for t in self.terms.iter() {
            let t_res = t.check(env.clone())?;
            let t_ty = t_res.ret_ty();
            premises.push(t_res);

            let t_norm;
            if features.normalizing {
                let t_norm_res = t_ty.normalize(env.clone());
                t_norm = t_norm_res.ret_ty();
                premises.push(t_norm_res);
            } else {
                t_norm = t_ty;
            }

            if features.kinded {
                let ty_knd = t_norm.check_kind(env.clone())?;
                match knd {
                    None => {
                        knd = Some(ty_knd);
                    }
                    Some(ref knd) => {
                        ty_knd.check_equal(knd)?;
                    }
                }
            }
            tys.push(t_norm);
        }

        let conc = TypingConclusion::new(env, self.clone(), TupleTy::<Lang>::new(tys));
        let deriv = TypingDerivation::tuple(conc, premises);
        Ok(deriv.into())
    }
}
