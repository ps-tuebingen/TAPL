use super::{LatexConfig, LatexFmt};
use derivation::{DefinitionDerivation, ProgramDerivation, TypingDerivation};
use syntax::{terms::Term, types::Type};

impl<T, Ty> LatexFmt for ProgramDerivation<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = if conf.include_envs {
            ("\\[", "\\]")
        } else {
            ("", "")
        };

        let mut def_strs = vec![];
        for def in self.def_derivations.iter() {
            def_strs.push(def.to_latex(conf))
        }

        let main_str = if let Some(ref md) = self.main_derivation {
            md.to_latex(conf)
        } else {
            "".to_owned()
        };

        format!(
            "{env_start} {} \\\\ \\\\{main_str} {env_end}",
            def_strs.join("\\\\\\\\")
        )
    }
}

impl<T, Ty> LatexFmt for DefinitionDerivation<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = if conf.include_envs {
            ("\\[", "\\]")
        } else {
            ("", "")
        };

        conf.include_envs = false;
        let body_str = self.body_derivation.to_latex(conf);

        format!(
            "{env_start}{body_str}\\UnaryInfC{{\\vdash {}:{}}}{env_end}",
            self.name,
            self.body_derivation.ty()
        )
    }
}
impl<T, Ty> LatexFmt for TypingDerivation<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        if conf.use_frac_array {
            derivation_to_frac_array(self, conf)
        } else {
            derivation_to_bussproofs(self, conf)
        }
    }
}

fn derivation_to_bussproofs<T, Ty>(
    deriv: &TypingDerivation<T, Ty>,
    conf: &mut LatexConfig,
) -> String
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    let conc_str = match deriv.premises.len() {
        0 => format!("\\UnaryInfC{{${}$}}", deriv.conc.to_latex(conf)),
        1 => format!("\\UnaryInfC{{${}$}}", deriv.conc.to_latex(conf)),
        2 => format!("\\BinaryInfC{{${}$}}", deriv.conc.to_latex(conf)),
        3 => format!("\\TrinaryInfC{{${}$}}", deriv.conc.to_latex(conf)),
        4 => format!("\\QuaternaryInfC{{${}$}}", deriv.conc.to_latex(conf)),
        5 => format!("\\QuinaryInfC{{${}$}}", deriv.conc.to_latex(conf)),
        _ => panic!("Derivations with more than 5 premises are not supported"),
    };

    let mut out = "".to_owned();

    if conf.include_envs {
        out += "\\begin{prooftree}\n";
    }

    if deriv.premises.is_empty() {
        out += "\\AxiomC{\\quad}\n";
    } else {
        let old_inc = conf.include_envs;
        conf.include_envs = false;
        for prem in deriv.premises.iter() {
            out += "\t";
            out += &prem.to_latex(conf);
            out += "\n";
        }
        conf.include_envs = old_inc;
    }

    out += "\\RightLabel{";
    out += &deriv.label.to_string();
    out += "}\n";
    out += &conc_str;
    if conf.include_envs {
        out += "\n\\end{prooftree}";
    }

    out
}

fn derivation_to_frac_array<T, Ty>(
    deriv: &TypingDerivation<T, Ty>,
    conf: &mut LatexConfig,
) -> String
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    let mut premise_str;
    if deriv.premises.is_empty() {
        premise_str = "\\quad".to_owned();
    } else {
        let cs = (0..deriv.premises.len())
            .map(|_| "c")
            .collect::<Vec<&str>>();
        premise_str = format!("\\begin{{array}}{{ {} }}", cs.join(" "));
        let inc_old = conf.include_envs;
        conf.include_envs = false;

        let mut premise_strs = vec![];
        for premise in deriv.premises.iter() {
            premise_strs.push(format!("\\displaystyle {}", premise.to_latex(conf)));
        }
        premise_str += &premise_strs.join("&");
        conf.include_envs = inc_old;
        premise_str += "\\end{array}";
    }

    let mut out = format!(
        "\n\\frac{{ \\displaystyle {} }}{{ {} }} \\quad \\text{{ {} }}\n",
        premise_str,
        deriv.conc.to_latex(conf),
        deriv.label
    );
    if conf.include_envs {
        out = format!("\\[{out}\\]");
    }
    out
}
