use crate::{
    check::{CheckKind, Env},
    errors::Error,
    syntax::{kinds::Kind, types::RecTy},
};

impl CheckKind for RecTy {
    fn check_kind(&self, env: &mut Env) -> Result<Kind, Error> {
        for (_, ty) in self.records.iter() {
            let ty_kind = ty.check_kind(&mut env.clone())?;
            ty_kind
                .check_equal(&Kind::Star)
                .map_err(|knd| Error::kinding(knd, self))?;
        }
        Ok(Kind::Star)
    }
}
