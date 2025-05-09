use super::{is_subtype, to_check_err, Env};
use crate::{
    lookup::{lookup_fields, lookup_method_type},
    syntax::{ClassName, ClassTable, Term},
};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};

impl<'a> Typecheck<'a> for Term {
    type Type = ClassName;
    type Env = (&'a mut Env, &'a ClassTable);

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check((&mut Default::default(), &Default::default()))
    }

    fn check(&self, (env, ct): Self::Env) -> Result<Self::Type, Error> {
        match self {
            Term::Var(v) => env
                .get(v)
                .cloned()
                .ok_or(to_check_err(ErrorKind::FreeVariable(v.clone()))),
            Term::Const(_) => Ok("Int".to_owned()),
            Term::FieldProjection(t, field) => {
                let ty = t.check((env, ct))?;
                let fields = lookup_fields(&ty, ct).map_err(to_check_err)?;
                let (ret_ty, _) = fields
                    .iter()
                    .find(|(_, field_name)| field_name == field)
                    .ok_or(to_check_err(ErrorKind::UndefinedName(field.clone())))?;
                Ok(ret_ty.clone())
            }
            Term::MethodCall(t, method, args) => {
                let obj_ty = t.check((&mut env.clone(), ct))?;
                let method_ty = lookup_method_type(method, &obj_ty, ct).map_err(to_check_err)?;
                if args.len() != method_ty.args.len() {
                    return Err(to_check_err(ErrorKind::Arity {
                        found: args.len(),
                        expected: method_ty.args.len(),
                    }));
                }
                let arg_tys = args
                    .iter()
                    .map(|arg| arg.check((&mut env.clone(), ct)))
                    .collect::<Result<Vec<ClassName>, Error>>()?;
                for (found_ty, expected_ty) in arg_tys.into_iter().zip(method_ty.args.into_iter()) {
                    if !is_subtype(&found_ty, &expected_ty, ct) {
                        return Err(to_check_err(ErrorKind::Subtype {
                            sub: found_ty,
                            sup: expected_ty,
                        }));
                    }
                }
                Ok(method_ty.ret)
            }
            Term::New(class, args) => {
                let fields = lookup_fields(class, ct).map_err(to_check_err)?;
                if fields.len() != args.len() {
                    return Err(to_check_err(ErrorKind::Arity {
                        found: args.len(),
                        expected: fields.len(),
                    }));
                }
                let arg_tys = args
                    .iter()
                    .map(|arg| arg.check((&mut env.clone(), ct)))
                    .collect::<Result<Vec<ClassName>, Error>>()?;
                for ((expected_ty, _), found_ty) in fields.into_iter().zip(arg_tys.into_iter()) {
                    if !is_subtype(&found_ty, &expected_ty, ct) {
                        return Err(to_check_err(ErrorKind::Subtype {
                            sub: found_ty,
                            sup: expected_ty,
                        }));
                    }
                }
                Ok(class.clone())
            }
            Term::Cast(class, t) => {
                let ty = t.check((env, ct))?;
                //T-DCast
                //T-SCast
                if is_subtype(&ty, class, ct) || (is_subtype(class, &ty, ct) && ty != *class) {
                    Ok(class.clone())
                //T-SCast
                } else {
                    println!("Warning: casting {ty} to {class}, where neither {ty}<:{class} nor {class}<:{ty}");
                    Ok(class.clone())
                }
            }
        }
    }
}
