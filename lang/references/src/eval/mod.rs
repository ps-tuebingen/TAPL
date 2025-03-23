use super::terms::{Loc, Term};
use std::collections::HashMap;

pub mod errors;
pub mod values;
use errors::Error;
use values::Value;

pub type Store = HashMap<Loc, Value>;

pub fn fresh_loc(st: &Store) -> Loc {
    let mut next_loc = 0;
    while st.contains_key(&next_loc) {
        next_loc += 1;
    }
    next_loc
}

pub fn eval(t: Term, st: &mut Store) -> Result<Value, Error> {
    match t {
        Term::Var(v) => Err(Error::FreeVar(v)),
        Term::Const(i) => Ok(Value::Const(i)),
        Term::Lambda { var, annot, body } => Ok(Value::Lambda {
            var,
            annot,
            body: *body,
        }),
        Term::App { fun, arg } => match (
            Value::from_term(*fun.clone()),
            Value::from_term(*arg.clone()),
        ) {
            (
                Ok(Value::Lambda {
                    var,
                    annot: _,
                    body,
                }),
                Ok(val),
            ) => {
                let body_subst = body.subst(&var, val.into());
                eval(body_subst, st)
            }
            (Ok(fun_val), Err(_)) => {
                let arg_evaled = eval(*arg, st)?;
                eval(
                    Term::App {
                        fun: Box::new(fun_val.into()),
                        arg: Box::new(arg_evaled.into()),
                    },
                    st,
                )
            }
            (Ok(val), _) => Err(Error::NotAFunction(val)),
            (Err(_), _) => {
                let fun_evaled = eval(*fun, st)?;
                eval(
                    Term::App {
                        fun: Box::new(fun_evaled.into()),
                        arg,
                    },
                    st,
                )
            }
        },
        Term::Unit => Ok(Value::Unit),
        Term::Ref(t) => match Value::from_term((*t).clone()) {
            Ok(val) => {
                let new_loc = fresh_loc(st);
                st.insert(new_loc, val);
                Ok(Value::Loc(new_loc))
            }
            Err(_) => {
                let t_evaled = eval(*t, st)?;
                eval(Term::Ref(Box::new(t_evaled.into())), st)
            }
        },
        Term::Deref(t) => match Value::from_term((*t).clone()) {
            Ok(Value::Loc(loc)) => st.get(&loc).ok_or(Error::LocationNotFound(loc)).cloned(),
            Ok(val) => Err(Error::NotALocation(val)),
            Err(_) => {
                let t_evaled = eval(*t, st)?;
                eval(Term::Deref(Box::new(t_evaled.into())), st)
            }
        },
        Term::Assign { to, body } => match (
            Value::from_term((*to).clone()),
            Value::from_term((*body).clone()),
        ) {
            (Ok(Value::Loc(loc)), Ok(val2)) => {
                st.insert(loc, val2);
                Ok(Value::Unit)
            }
            (Ok(val1), Ok(_)) => Err(Error::NotALocation(val1)),
            (Err(_), _) => {
                let to_evaled = eval(*to, st)?;
                eval(
                    Term::Assign {
                        to: Box::new(to_evaled.into()),
                        body,
                    },
                    st,
                )
            }
            (Ok(val1), Err(_)) => {
                let body_evaled = eval(*body, st)?;
                eval(
                    Term::Assign {
                        to: Box::new(val1.into()),
                        body: Box::new(body_evaled.into()),
                    },
                    st,
                )
            }
        },
        Term::Loc(loc) => Ok(Value::Loc(loc)),
    }
}

#[cfg(test)]
mod check_tests {
    use super::{eval, Term, Value};
    use crate::types::Type;

    #[test]
    fn eval1() {
        let result = eval(
            Term::app(
                Term::lam(
                    "x",
                    Type::Ref(Box::new(Type::Unit)),
                    Term::deref("x".into()),
                ),
                Term::app(
                    Term::lam("y", Type::Unit, Term::reft("y".into())),
                    Term::Unit,
                ),
            ),
            &mut Default::default(),
        )
        .unwrap();
        let expected = Value::Unit;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval2() {
        let result = eval(
            Term::App {
                fun: Box::new(Term::Lambda {
                    var: "x".to_owned(),
                    annot: Type::Ref(Box::new(Type::Unit)),
                    body: Box::new(Term::Assign {
                        to: Box::new(Term::Var("x".to_owned())),
                        body: Box::new(Term::Deref(Box::new(Term::Var("x".to_owned())))),
                    }),
                }),
                arg: Box::new(Term::Ref(Box::new(Term::Unit))),
            },
            &mut Default::default(),
        )
        .unwrap();
        let expected = Value::Unit;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_store() {
        let result = eval(
            Term::seq(
                Term::assign(
                    Term::reft(Term::Unit),
                    Term::app(Term::lam("x", Type::Unit, "x".into()), Term::Unit),
                ),
                Term::deref(0.into()),
            ),
            &mut Default::default(),
        )
        .unwrap();
        let expected = Value::Unit;
        assert_eq!(result, expected)
    }
}
