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
        let features = Lang::features();
        let mut premises = vec![];

        let term_res = self.record.check(env.clone())?;
        let term_ty = term_res.ret_ty();
        premises.push(term_res);

        let ty_norm;
        if features.normalizing {
            let ty_norm_deriv = term_ty.normalize(env.clone());
            ty_norm = ty_norm_deriv.ret_ty();
            premises.push(ty_norm_deriv);
        } else {
            ty_norm = term_ty;
        }

        if features.kinded {
            ty_norm.check_kind(env.clone())?;
        }

        let term_rec = match ty_norm.clone().into_variable() {
            Ok(v) => env.get_tyvar_super(&v.v)?,
            Err(_) => ty_norm,
        };

        let term_rec_norm;
        if features.normalizing {
            let term_rec_norm_deriv = term_rec.normalize(env.clone());
            term_rec_norm = term_rec_norm_deriv.ret_ty();
            premises.push(term_rec_norm_deriv);
        } else {
            term_rec_norm = term_rec;
        }

        let rec_ty = term_rec_norm.into_record()?;
        let ty = rec_ty
            .records
            .get(&self.label)
            .ok_or(UndefinedLabel::new(&self.label))
            .cloned()?;

        let conc = TypingConclusion::new(env, self.clone(), ty);
        let deriv = TypingDerivation::recordproj(conc, premises);
        Ok(deriv.into())
    }
}
