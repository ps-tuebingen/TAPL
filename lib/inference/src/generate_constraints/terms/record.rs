use super::{GenState, GenerateConstraints};
use std::collections::HashMap;
use syntax::{language::Language, terms::Record, types::Record as RecordTy};

impl<Lang> GenerateConstraints for Record<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    RecordTy<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let mut records = HashMap::new();
        for (lab, t) in &self.records {
            let rec_ty = t.generate_constraints(state);
            records.insert(lab.clone(), rec_ty);
        }
        RecordTy::new(records, self.span).into()
    }
}
