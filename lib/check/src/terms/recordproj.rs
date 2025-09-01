use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::UndefinedLabel;
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::RecordProj, types::TypeGroup};

impl<Lang> Typecheck for RecordProj<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type:
        TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let term_res = self.record.check(env.clone())?;
        let term_ty = term_res.ret_ty().normalize(env.clone());
        term_ty.check_kind(env.clone())?;

        let term_rec = match term_ty.clone().into_variable() {
            Ok(v) => env.get_tyvar_super(&v.v)?.normalize(env.clone()),
            Err(_) => term_ty,
        };
        let rec_ty = term_rec.into_record()?;
        let ty = rec_ty
            .records
            .get(&self.label)
            .ok_or(UndefinedLabel::new(&self.label))
            .cloned()?;

        let conc = TypingConclusion::new(env, self.clone(), ty);
        let deriv = TypingDerivation::recordproj(conc, term_res);
        Ok(deriv.into())
    }
}
