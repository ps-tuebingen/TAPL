use super::{
    errors::Error,
    lookup::lookup_method_body,
    syntax::{ClassName, ClassTable, Term},
    typing::is_subtype,
};
use common::Eval;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Const(i64),
    Object {
        class_name: ClassName,
        args: Vec<Value>,
    },
}

impl Value {
    pub fn into_object(self) -> Result<(String, Vec<Value>), Error> {
        if let Value::Object { class_name, args } = self {
            Ok((class_name, args))
        } else {
            Err(Error::BadValue {
                found: self,
                expected: "Object Creation".to_owned(),
            })
        }
    }
}

impl From<Value> for Term {
    fn from(v: Value) -> Term {
        match v {
            Value::Const(i) => Term::Const(i),
            Value::Object { class_name, args } => {
                Term::New(class_name, args.into_iter().map(|arg| arg.into()).collect())
            }
        }
    }
}

impl Eval for Term {
    type Value = Value;
    type Error = Error;
    type Env = ClassTable;

    fn eval(self, ct: &mut ClassTable) -> Result<Value, Error> {
        match self {
            Term::Var(v) => Err(Error::FreeVar(v)),
            Term::Const(i) => Ok(Value::Const(i)),
            Term::FieldProjection(t, field) => {
                let obj_val = t.eval(ct)?;
                let (class_name, args) = obj_val.into_object()?;
                let decl = ct
                    .classes
                    .get(&class_name)
                    .ok_or(Error::ClassNotFound(class_name.clone()))?;
                let (index, _) = decl
                    .fields
                    .iter()
                    .enumerate()
                    .find(|(_, (_, name))| *name == field)
                    .ok_or(Error::FieldNotFound {
                        class: class_name.clone(),
                        field,
                    })?;
                args.get(index)
                    .ok_or(Error::ConstructorArity {
                        class: class_name,
                        found: args.len(),
                        expected: decl.constructor.num_args(),
                    })
                    .cloned()
            }
            Term::MethodCall(t, name, args) => {
                let obj_val = t.eval(ct)?;
                let mut arg_vals = vec![];
                for arg in args {
                    arg_vals.push(arg.eval(ct)?);
                }
                let arg_terms: Vec<Term> = arg_vals.into_iter().map(|arg| arg.into()).collect();
                let (class_name, ctor_args) = obj_val.into_object()?;
                let (arg_names, method_body) = lookup_method_body(&name, &class_name, ct)?;
                if arg_names.len() != arg_terms.len() {
                    return Err(Error::MethodArity {
                        method: name,
                        class: class_name,
                        found: arg_terms.len(),
                        expected: arg_names.len(),
                    });
                }

                let mut subst_map = HashMap::from_iter(arg_names.into_iter().zip(arg_terms));
                subst_map.insert(
                    "this".to_owned(),
                    Term::New(
                        class_name,
                        ctor_args.into_iter().map(|arg| arg.into()).collect(),
                    ),
                );
                method_body.subst(&subst_map).eval(ct)
            }
            // E-New-Arg
            Term::New(class, args) => {
                let mut arg_vals = vec![];
                for arg in args {
                    arg_vals.push(arg.eval(ct)?);
                }
                Ok(Value::Object {
                    class_name: class,
                    args: arg_vals,
                })
            }
            Term::Cast(to_class, t) => {
                let obj_val = t.eval(ct)?;
                let (class_name, args) = obj_val.into_object()?;
                if is_subtype(&class_name, &to_class, ct) {
                    Ok(Value::Object { class_name, args })
                } else {
                    Err(Error::NotASubClass {
                        sub: class_name,
                        sup: to_class,
                    })
                }
            }
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Value as Into<Term>>::into(self.clone()).fmt(f)
    }
}

#[cfg(test)]
mod eval_tests {
    use super::{Term, Value};
    use crate::test_common::example_table;
    use common::Eval;

    #[test]
    fn eval_setfirst() {
        let result = Term::MethodCall(
            Box::new(Term::New(
                "Pair".to_owned(),
                vec![
                    Term::New("A".to_owned(), vec![]),
                    Term::New("B".to_owned(), vec![]),
                ],
            )),
            "setfst".to_owned(),
            vec![Term::New("B".to_owned(), vec![])],
        )
        .eval(&mut example_table())
        .unwrap();
        let expected = Value::Object {
            class_name: "Pair".to_owned(),
            args: vec![
                Value::Object {
                    class_name: "B".to_owned(),
                    args: vec![],
                },
                Value::Object {
                    class_name: "B".to_owned(),
                    args: vec![],
                },
            ],
        };
        assert_eq!(result, expected)
    }
}
