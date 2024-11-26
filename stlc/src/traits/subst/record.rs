use super::Subst;
use crate::{
    syntax::{Record, RecordProj, Term},
    Var,
};

impl Subst for Record {
    type Target = Record;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Record {
            records: self
                .records
                .into_iter()
                .map(|(label, t)| (label, t.subst(var.clone(), term.clone())))
                .collect(),
        }
    }
}

impl Subst for RecordProj {
    type Target = RecordProj;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        RecordProj {
            record: self.record.subst(var, term),
            label: self.label,
        }
    }
}

#[cfg(test)]
mod record_tests {
    use super::{Record, RecordProj, Subst};
    use std::collections::HashMap;

    #[test]
    fn subst_rec() {
        let result = Record {
            records: HashMap::from([("x".to_owned(), "x".to_owned().into())]),
        }
        .subst("x".to_owned(), "y".to_owned().into());
        let expected = Record {
            records: HashMap::from([("x".to_owned(), "y".to_owned().into())]),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_proj() {
        let result = RecordProj {
            label: "x".to_owned(),
            record: Box::new("x".to_owned().into()),
        }
        .subst("x".to_owned(), "y".to_owned().into());
        let expected = RecordProj {
            label: "x".to_owned(),
            record: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }
}
