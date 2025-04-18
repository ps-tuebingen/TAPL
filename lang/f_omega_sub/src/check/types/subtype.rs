use crate::{check::Env, syntax::types::Type, to_err, traits::SubstTy};
use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    Typecheck,
};

pub fn to_subty_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Subtyping)
}

pub fn check_subtype(lower: &Type, upper: &Type, env: &mut Env) -> Result<(), Error> {
    let lower_kind = lower.check(&mut env.clone())?;
    let _ = upper.check(&mut env.clone())?;

    if let Type::Top(knd) = upper {
        lower_kind.check_equal(knd).map_err(to_subty_err)?;
        return Ok(());
    }

    let mut upper = upper.clone();
    if let Type::Var(ref v) = upper {
        let ty = env.get_tyvar(v).map_err(to_subty_err)?;
        upper = ty;
    }

    match lower {
        Type::Var(v) => {
            let ty = env.get_tyvar(v).map_err(to_subty_err)?;
            check_subtype(&ty, &upper, env)
        }
        Type::Fun(fun) => {
            let fun2 = upper.clone().as_fun().map_err(to_subty_err)?;
            check_subtype(&fun2.from, &fun.from, &mut env.clone())?;
            check_subtype(&fun.to, &fun2.to, env)
        }
        Type::Universal(uni) => {
            let uni2 = upper.clone().as_universal().map_err(to_subty_err)?;
            let _ = uni.sup_ty.check(&mut env.clone())?;
            uni.sup_ty.check_equal(&uni2.sup_ty).map_err(to_subty_err)?;
            env.add_tyvar(&uni.var, &uni.sup_ty)?;
            let uni2_subst = uni2.ty.subst_ty(&uni2.var, uni.var.as_str().into());
            check_subtype(&uni.ty, &uni2_subst, env)
        }
        Type::Existential(ex) => {
            let ex2 = upper.clone().as_existential().map_err(to_subty_err)?;
            let _ = ex.sup_ty.check(&mut env.clone())?;
            ex.sup_ty.check_equal(&ex2.sup_ty).map_err(to_subty_err)?;
            env.add_tyvar(&ex.var, &ex.sup_ty)?;
            let ex2_subst = ex2.ty.subst_ty(&ex2.var, ex.var.as_str().into());
            check_subtype(&ex.ty, &ex2_subst, env)
        }
        Type::OpLambda(lam) => {
            let lam2 = upper.clone().as_oplambda().map_err(to_subty_err)?;
            lam.annot.check_equal(&lam2.annot).map_err(to_subty_err)?;
            env.add_tyvar(&lam.var, &Type::Top(lam.annot.clone()))?;
            let lam2_subst = lam2.body.subst_ty(&lam2.var, lam2.var.as_str().into());
            check_subtype(&lam.body, &lam2_subst, env)
        }
        Type::OpApp(app) => {
            let app2 = upper.clone().as_opapp().map_err(to_subty_err)?;
            check_subtype(&app.fun, &app2.fun, env)
        }
        Type::Record(rec) => {
            let rec2 = upper.clone().as_rec().map_err(to_subty_err)?;
            for (label, ty2) in rec2.records.iter() {
                let ty = rec
                    .records
                    .get(label)
                    .ok_or(to_subty_err(ErrorKind::UndefinedLabel(label.clone())))?;
                check_subtype(ty, ty2, &mut env.clone())?;
            }
            Ok(())
        }
        Type::Nat => {
            if matches!(upper, Type::Nat) {
                Ok(())
            } else {
                Err(to_subty_err(ErrorKind::TypeMismatch {
                    found: upper.to_string(),
                    expected: "Nat".to_owned(),
                }))
            }
        }
        Type::Top(_) => Err(to_subty_err(ErrorKind::TypeMismatch {
            found: upper.to_string(),
            expected: "Top".to_owned(),
        })),
    }
}
