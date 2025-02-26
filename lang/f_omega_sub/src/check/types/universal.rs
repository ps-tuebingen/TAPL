use crate::{
    check::{Check, Env},
    errors::Error,
    syntax::{kinds::Kind, types::Universal},
};

impl Check for Universal {
    type Target = Kind;
    fn check(&self, env: &mut Env) -> Result<Self::Target, Error> {
        env.add_tyvar(&self.var, &self.sup_ty)?;
        let ty_kind = self.ty.check(env)?;
        ty_kind
            .check_equal(&Kind::Star)
            .map_err(|knd| Error::kind(knd, self))?;
        Ok(Kind::Star)
    }
}
