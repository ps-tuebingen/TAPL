use crate::{LatexConfig, LatexFmt};
use grammar::symbols::Keyword;

impl LatexFmt for Keyword {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        match self {
            Self::As => format!("{env_start} \\text{{ as }} {env_end}"),
            Self::In => format!("{env_start} \\text{{ in }} {env_end}"),
            Self::Let => format!("{env_start} \\text{{ let }}{env_end}"),
            Self::If => format!("{env_start} \\text{{ if }}{env_end}"),
            Self::Else => format!("{env_start} \\text{{ else }}{env_end}"),
            Self::Try => format!("{env_start}\\text{{ try }} {env_end}"),
            Self::Catch => format!("{env_start}\\text{{ catch }} {env_end}"),
            Self::Nil => format!("{env_start} \\text{{ nil }}{env_end}"),
            Self::Cons => format!("{env_start}\\text{{ cons }} {env_end}"),
            Self::Head => format!("{env_start}\\text{{ head }} {env_end}"),
            Self::Tail => format!("{env_start}\\text{{ tail }} {env_end}"),
            Self::IsNil => format!("{env_start}\\text{{ isnil }} {env_end}"),
            Self::Succ => format!("{env_start}\\text{{ succ }} {env_end}"),
            Self::Pred => format!("{env_start}\\text{{ pred }} {env_end}"),
            Self::IsZero => format!("{env_start}\\text{{ iszero }} {env_end}"),
            Self::Something => format!("{env_start}\\text{{ something }} {env_end}"),
            Self::Left => format!("{env_start}\\text{{ inl }} {env_end}"),
            Self::Right => format!("{env_start}\\text{{ inr }} {env_end}"),
            Self::Nothing => format!("{env_start}\\text{{ nothing }} {env_end}"),
            Self::Raise => format!("{env_start}\\text{{ raise }} {env_end}"),
            Self::Err => format!("{env_start}\\text{{ error }} {env_end}"),
            Self::Fold => format!("{env_start}\\text{{ fold }} {env_end}"),
            Self::Unfold => format!("{env_start}\\text{{ unfold }} {env_end}"),
            Self::Ref => format!("{env_start}\\text{{ ref }} {env_end}"),
            Self::Fix => format!("{env_start}\\text{{ fix }} {env_end}"),
            Self::Unit => format!("{env_start}\\text{{ unit }} {env_end}"),
            Self::True => format!("{env_start}\\text{{ true }} {env_end}"),
            Self::False => format!("{env_start}\\text{{ false }} {env_end}"),
            Self::Fst => format!("{env_start}\\text{{ fst }} {env_end}"),
            Self::Snd => format!("{env_start}\\text{{ snd }} {env_end}"),
            Self::Source => format!("{env_start}\\text{{ Source }} {env_end}"),
            Self::Sink => format!("{env_start}\\text{{ Sink }} {env_end}"),
            Self::Reference => format!("{env_start}\\text{{ Ref }} {env_end}"),
            Self::Optional => format!("{env_start}\\text{{ Optional }} {env_end}"),
            Self::List => format!("{env_start}\\text{{ List }} {env_end}"),
            Self::UnitTy => format!("{env_start}\\text{{ Unit }} {env_end}"),
            Self::Nat => format!("{env_start}\\text{{ Nat }} {env_end}"),
            Self::Bool => format!("{env_start}\\text{{ Bool }} {env_end}"),
            Self::Case => format!("{env_start}\\text{{ case }} {env_end}"),
            Self::Of => format!("{env_start} \\text{{ of }} {env_end}"),
            Self::With => format!("{env_start}\\text{{with}}{env_end}"),
        }
    }
}
