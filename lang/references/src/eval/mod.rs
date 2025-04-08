use super::terms::{Cmp, Loc, Term};
use common::Eval;
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

impl Eval for Term {
    type Value = Value;
    type Error = Error;
    type Env = Store;

    fn eval(self, st: &mut Store) -> Result<Value, Error> {
        match self {
            Term::Var(v) => Err(Error::FreeVar(v)),
            Term::Const(i) => Ok(Value::Const(i)),
            Term::Succ(t) => {
                let val = t.eval(st)?;
                if let Value::Const(i) = val {
                    Ok(Value::Const(i + 1))
                } else {
                    Err(Error::NotANumber(val))
                }
            }
            Term::Pred(t) => {
                let val = t.eval(st)?;
                if let Value::Const(i) = val {
                    Ok(Value::Const(i - 1))
                } else {
                    Err(Error::NotANumber(val))
                }
            }
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
                    body_subst.eval(st)
                }
                (Ok(fun_val), Err(_)) => {
                    let arg_evaled = arg.eval(st)?;

                    Term::App {
                        fun: Box::new(fun_val.into()),
                        arg: Box::new(arg_evaled.into()),
                    }
                    .eval(st)
                }
                (Ok(val), _) => Err(Error::NotAFunction(val)),
                (Err(_), _) => {
                    let fun_evaled = fun.eval(st)?;
                    Term::App {
                        fun: Box::new(fun_evaled.into()),
                        arg,
                    }
                    .eval(st)
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
                    let t_evaled = t.eval(st)?;
                    Term::Ref(Box::new(t_evaled.into())).eval(st)
                }
            },
            Term::Deref(t) => match Value::from_term((*t).clone()) {
                Ok(Value::Loc(loc)) => st.get(&loc).ok_or(Error::LocationNotFound(loc)).cloned(),
                Ok(val) => Err(Error::NotALocation(val)),
                Err(_) => {
                    let t_evaled = t.eval(st)?;
                    Term::Deref(Box::new(t_evaled.into())).eval(st)
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
                    let to_evaled = to.eval(st)?;
                    Term::Assign {
                        to: Box::new(to_evaled.into()),
                        body,
                    }
                    .eval(st)
                }
                (Ok(val1), Err(_)) => {
                    let body_evaled = body.eval(st)?;
                    Term::Assign {
                        to: Box::new(val1.into()),
                        body: Box::new(body_evaled.into()),
                    }
                    .eval(st)
                }
            },
            Term::Loc(loc) => Ok(Value::Loc(loc)),
            Term::Let {
                var,
                bound_term,
                in_term,
            } => {
                let bound_val = bound_term.eval(st)?;
                let in_subst = in_term.subst(&var, bound_val.into());
                in_subst.eval(st)
            }
            Term::If {
                left,
                cmp,
                right,
                then_term,
                else_term,
            } => {
                let left_val = left.eval(st)?;
                let left_num = if let Value::Const(i) = left_val {
                    i
                } else {
                    return Err(Error::NotANumber(left_val));
                };
                let right_val = right.eval(st)?;
                let right_num = if let Value::Const(i) = right_val {
                    i
                } else {
                    return Err(Error::NotANumber(right_val));
                };

                let cmp_fun = |l, r| match cmp {
                    Cmp::Equal => l == r,
                    Cmp::Less => l < r,
                    Cmp::LessEqual => l <= r,
                    Cmp::Greater => l > r,
                    Cmp::GreaterEqual => l >= r,
                };

                if cmp_fun(left_num, right_num) {
                    then_term.eval(st)
                } else {
                    else_term.eval(st)
                }
            }
        }
    }
}

#[cfg(test)]
mod check_tests {
    use super::{Eval, Term, Value};
    use crate::types::Type;

    #[test]
    fn eval1() {
        let result = Term::app(
            Term::lam(
                "x",
                Type::Ref(Box::new(Type::Unit)),
                Term::deref("x".into()),
            ),
            Term::app(
                Term::lam("y", Type::Unit, Term::reft("y".into())),
                Term::Unit,
            ),
        )
        .eval(&mut Default::default())
        .unwrap();
        let expected = Value::Unit;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval2() {
        let result = Term::App {
            fun: Box::new(Term::Lambda {
                var: "x".to_owned(),
                annot: Type::Ref(Box::new(Type::Unit)),
                body: Box::new(Term::Assign {
                    to: Box::new(Term::Var("x".to_owned())),
                    body: Box::new(Term::Deref(Box::new(Term::Var("x".to_owned())))),
                }),
            }),
            arg: Box::new(Term::Ref(Box::new(Term::Unit))),
        }
        .eval(&mut Default::default())
        .unwrap();
        let expected = Value::Unit;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_store() {
        let result = Term::seq(
            Term::assign(
                Term::reft(Term::Unit),
                Term::app(Term::lam("x", Type::Unit, "x".into()), Term::Unit),
            ),
            Term::deref(0.into()),
        )
        .eval(&mut Default::default())
        .unwrap();
        let expected = Value::Unit;
        assert_eq!(result, expected)
    }
}
