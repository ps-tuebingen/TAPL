use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{LambdaSub, Term},
    types::{ForallBounded, Type},
};

impl<T, Ty> Typecheck for LambdaSub<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T>,
    Ty: Type + Kindcheck<Ty> + Normalize<Ty>,
    ForallBounded<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;

    fn check(
        &self,
        mut env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        let sup_norm = self.sup_ty.clone().normalize(env.clone());
        let sup_kind = sup_norm.check_kind(env.clone())?;
        env.add_tyvar_super(self.var.clone(), sup_norm.clone());
        env.add_tyvar_kind(self.var.clone(), sup_kind.clone());
        let term_res = self.body.check(env.clone())?;
        let term_ty = term_res.ret_ty().normalize(env.clone());

        let conc = TypingConclusion::new(
            env,
            self.clone(),
            ForallBounded::new(&self.var, self.sup_ty.clone(), term_ty).into(),
        );
        let deriv = TypingDerivation::lambdasub(conc, term_res);

        Ok(deriv.into())
    }
}
