use super::Eval;
use crate::{
    errors::{Error, ErrorKind},
    terms::{Record, RecordProj, Term},
    traits::is_value::IsValue,
};

impl Eval for Record {
    fn eval_once(self) -> Result<Term, Error> {
        let mut recs = self.records.clone();
        for (label, term) in self.records.iter() {
            if !term.is_value() {
                let term_evaled = term.clone().eval_once()?;
                recs.insert(label.clone(), term_evaled);
            }
        }
        Ok(Record { records: recs }.into())
    }
}

impl Eval for RecordProj {
    fn eval_once(self) -> Result<Term, Error> {
        if self.record.is_value() {
            let rec = self
                .record
                .as_record()
                .map_err(|knd| Error::eval(knd, &self))?;
            rec.records
                .get(&self.label)
                .ok_or(Error::eval(
                    ErrorKind::UndefinedLabel(self.label.clone()),
                    &self,
                ))
                .cloned()
        } else {
            let rec_evaled = self.record.eval_once()?;
            Ok(RecordProj {
                record: Box::new(rec_evaled),
                ty: self.ty,
                label: self.label,
            }
            .into())
        }
    }
}
