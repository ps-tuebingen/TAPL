use super::{to_err, Env};
use crate::{traits::SubstTy, types::Type};
use common::errors::{Error, ErrorKind, ErrorLocation};

fn to_sub_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Subtyping)
}

pub fn check_subtype(lower: Type, upper: Type, env: &mut Env) -> Result<(), Error> {
    if lower == upper {
        return Ok(());
    }
    if upper == Type::Top {
        return Ok(());
    }

    match lower {
        Type::Var(v) => {
            let sup = env.get_tyvar(&v).map_err(to_sub_err)?;
            sup.check_equal(&upper).map_err(to_sub_err)?;
            Ok(())
        }
        Type::Fun { from, to } => {
            let (from_upper, to_upper) = upper.as_fun().map_err(to_sub_err)?;
            check_subtype(from_upper, *from, &mut env.clone())?;
            check_subtype(*to, to_upper, env)
        }
        Type::Forall { var, sup_ty, ty } => {
            let (var_upper, upper_sup, upper_ty) = upper.as_forall().map_err(to_sub_err)?;
            sup_ty.check_equal(&upper_sup).map_err(to_sub_err)?;
            let super_subst = upper_ty.subst_ty(&var_upper, var.as_str().into());
            env.add_tyvar(&var, &sup_ty);
            check_subtype(*ty, super_subst, env)
        }
        Type::Exists { var, sup_ty, ty } => {
            let (var_upper, sup_ty_upper, ty_upper) = upper.as_exists().map_err(to_sub_err)?;
            sup_ty_upper.check_equal(&sup_ty).map_err(to_sub_err)?;
            let super_subst = ty_upper.subst_ty(&var_upper, var.as_str().into());
            env.add_tyvar(&var, &sup_ty);
            check_subtype(*ty, super_subst, env)
        }
        Type::Record(mut rec1) => {
            let rec2 = upper.as_record().map_err(to_sub_err)?;
            for (label, ty2) in rec2.into_iter() {
                let ty1 = if let Some(ty) = rec1.remove(&label) {
                    ty
                } else {
                    return Err(to_sub_err(ErrorKind::UndefinedLabel(label.clone())));
                };
                check_subtype(ty1.clone(), ty2, env)?;
            }
            Ok(())
        }
        Type::Nat => Err(to_sub_err(ErrorKind::TypeMismatch {
            found: Type::Nat.to_string(),
            expected: "Subtype".to_owned(),
        })),
        Type::Top => Err(to_sub_err(ErrorKind::TypeMismatch {
            found: Type::Top.to_string(),
            expected: "Subtype".to_owned(),
        })),
    }
}
