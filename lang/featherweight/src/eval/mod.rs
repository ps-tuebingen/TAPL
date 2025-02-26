use super::{
    errors::Error,
    lookup::lookup_method_body,
    syntax::{ClassTable, Term},
    typing::is_subtype,
};
use std::collections::HashMap;

pub fn eval(t: Term, ct: &ClassTable) -> Result<Term, Error> {
    let evaled = eval_once(t.clone(), ct)?;
    if t == evaled && evaled.is_value() {
        Ok(evaled)
    } else if t == evaled {
        Err(Error::StuckTerm(evaled))
    } else {
        eval(evaled, ct)
    }
}

fn eval_once(t: Term, ct: &ClassTable) -> Result<Term, Error> {
    match t {
        Term::Var(v) => Err(Error::FreeVar(v)),
        //E-Field
        Term::FieldProjection(t, field) if !t.is_value() => {
            let t_evaled = eval_once(*t, ct)?;
            Ok(Term::FieldProjection(Box::new(t_evaled), field))
        }
        // E-ProjNew
        Term::FieldProjection(v, field) => {
            if let Term::New(class, args) = *v {
                let decl = ct.get(&class).ok_or(Error::ClassNotFound(class.clone()))?;
                let (index, _) = decl
                    .fields
                    .iter()
                    .enumerate()
                    .find(|(_, (_, name))| *name == field)
                    .ok_or(Error::FieldNotFound {
                        class: class.clone(),
                        field,
                    })?;
                args.get(index)
                    .ok_or(Error::ConstructorArity {
                        class,
                        found: args.len(),
                        expected: decl.constructor.num_args(),
                    })
                    .cloned()
            } else {
                Err(Error::UnexpectedTerm {
                    found: *v,
                    expected: "Object Creation".to_owned(),
                })
            }
        }

        // E-Invk-Recv
        Term::MethodCall(t, name, args) if !t.is_value() => {
            let t_evaled = eval_once(*t, ct)?;
            Ok(Term::MethodCall(Box::new(t_evaled), name, args))
        }
        // E-Invk-Arg
        Term::MethodCall(t, name, args) if args.iter().any(|arg| !arg.is_value()) => {
            let mut args = args;
            let (first_non_val, _) = args
                .iter()
                .enumerate()
                .find(|(_, arg)| !arg.is_value())
                .unwrap();
            let to_eval = args[first_non_val].clone();
            let evaled = eval_once(to_eval, ct)?;
            args[first_non_val] = evaled;
            Ok(Term::MethodCall(t, name, args))
        }
        // E-InvkNew
        Term::MethodCall(v, name, args) => {
            if let Term::New(class, ctor_args) = *v {
                let (arg_vars, body) = lookup_method_body(&name, &class, ct)?;
                if arg_vars.len() != args.len() {
                    return Err(Error::MethodArity {
                        method: name,
                        class,
                        found: args.len(),
                        expected: arg_vars.len(),
                    });
                }

                let mut subst_map = HashMap::from_iter(arg_vars.into_iter().zip(args));
                subst_map.insert("this".to_owned(), Term::New(class, ctor_args));
                Ok(body.subst(&subst_map))
            } else {
                Err(Error::UnexpectedTerm {
                    found: *v,
                    expected: "Object Creation".to_owned(),
                })
            }
        }

        // E-New-Arg
        Term::New(class, args) if args.iter().any(|arg| !arg.is_value()) => {
            let mut args = args;
            let (first_non_val, _) = args
                .iter()
                .enumerate()
                .find(|(_, arg)| !arg.is_value())
                .unwrap();
            let to_eval = args[first_non_val].clone();
            let evaled = eval_once(to_eval, ct)?;
            args[first_non_val] = evaled;
            Ok(Term::New(class, args))
        }
        // Value
        Term::New(class, args) => Ok(Term::New(class, args)),
        // E-CastNew
        Term::Cast(to_class, t) if t.is_value() => {
            if let Term::New(class, args) = *t {
                if is_subtype(&class, &to_class, ct) {
                    Ok(Term::New(class, args))
                } else {
                    Err(Error::NotASubClass {
                        sub: class,
                        sup: to_class,
                    })
                }
            } else {
                Err(Error::UnexpectedTerm {
                    found: *t,
                    expected: "Object Creation".to_owned(),
                })
            }
        }
        // E-Cast
        Term::Cast(class, t) => {
            let evaled = eval_once(*t, ct)?;
            Ok(Term::Cast(class, Box::new(evaled)))
        }
    }
}

#[cfg(test)]
mod eval_tests {
    use super::{eval, Term};
    use crate::test_common::example_table;

    #[test]
    fn eval_setfirst() {
        let result = eval(
            Term::MethodCall(
                Box::new(Term::New(
                    "Pair".to_owned(),
                    vec![
                        Term::New("A".to_owned(), vec![]),
                        Term::New("B".to_owned(), vec![]),
                    ],
                )),
                "setfst".to_owned(),
                vec![Term::New("B".to_owned(), vec![])],
            ),
            &example_table(),
        )
        .unwrap();
        let expected = Term::New(
            "Pair".to_owned(),
            vec![
                Term::New("B".to_owned(), vec![]),
                Term::New("B".to_owned(), vec![]),
            ],
        );
        assert_eq!(result, expected)
    }
}
