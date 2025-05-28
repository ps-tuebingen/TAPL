use crate::{env::CheckEnvironment, to_check_err, Kindcheck, Normalize, Typecheck};
use common::errors::{Error, ErrorKind};
use syntax::{
    terms::{RecordProj, Term},
    types::TypeGroup,
};

impl<T> Typecheck for RecordProj<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>
        + Kindcheck<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self
            .record
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        term_ty.check_kind(&mut env.clone())?;

        let term_rec = match term_ty.clone().into_variable() {
            Ok(v) => env
                .get_tyvar_super(&v.v)
                .map_err(to_check_err)?
                .normalize(env),
            Err(_) => term_ty,
        };
        let rec_ty = term_rec.into_record().map_err(to_check_err)?;
        rec_ty
            .records
            .get(&self.label)
            .ok_or(to_check_err(ErrorKind::UndefinedLabel(self.label.clone())))
            .cloned()
    }
}
