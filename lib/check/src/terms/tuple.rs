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
        let mut tys: Vec<<Self::Lang as Language>::Type> = vec![];
        let mut ress = vec![];
        let mut knd = None;
        for t in self.terms.iter() {
            let t_res = t.check(env.clone())?;
            let t_ty = t_res.ret_ty().normalize(env.clone());
            ress.push(t_res);
            let ty_knd = t_ty.check_kind(env.clone())?;
            tys.push(t_ty);

            match knd {
                None => {
                    knd = Some(ty_knd);
                }
                Some(ref knd) => {
                    ty_knd.check_equal(knd)?;
                }
            }
        }

        let conc = TypingConclusion::new(env, self.clone(), TupleTy::<Lang>::new(tys));
        let deriv = TypingDerivation::tuple(conc, ress);
        Ok(deriv.into())
    }
}
