use crate::{errors::CheckError, Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, Derivation};
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
    type Type = <T as Typecheck>::Type;
    type Term = T;

    fn check(
        &self,
        mut env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError<Self::Type>> {
        let sup_norm = self.sup_ty.clone().normalize(env.clone());
        let sup_kind = sup_norm.check_kind(env.clone())?;
        env.add_tyvar_super(self.var.clone(), sup_norm.clone());
        env.add_tyvar_kind(self.var.clone(), sup_kind.clone());
        let term_res = self.body.check(env.clone())?;
        let term_ty = term_res.ty().normalize(env.clone());

        let conc = Conclusion::new(
            env,
            self.clone(),
            ForallBounded::new(&self.var, self.sup_ty.clone(), term_ty).into(),
        );
        let deriv = Derivation::lambdasub(conc, term_res);

        Ok(deriv)
    }
}
