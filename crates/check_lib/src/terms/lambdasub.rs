use crate::{Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{LambdaSub, Term},
    types::{ForallBounded, Type},
};

impl<T, Ty> Typecheck for LambdaSub<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    Ty: Type + Kindcheck<Ty> + Normalize<Ty>,
    ForallBounded<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(
        &self,
        mut env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Self::Deriv, CheckError> {
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
        let deriv = TypingDerivation::lambdasub(conc, term_res);

        Ok(deriv)
    }
}
