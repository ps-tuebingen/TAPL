use super::{LatexConfig, LatexFmt};
use derivation::{DefinitionDerivation, ProgramDerivation, TypingDerivation};
use syntax::{terms::Term, types::Type};

impl<T, Ty> LatexFmt for ProgramDerivation<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let mut def_strs = vec![];
        let old_inc = conf.include_envs;
        for def in self.def_derivations.iter() {
            def_strs.push(def.to_latex(conf));
            conf.include_envs = old_inc;
        }
        def_strs.push(self.main_derivation.to_latex(conf));
        def_strs.join("\n\\quad \\\\ \\quad \\\\\n")
    }
}

impl<T, Ty> LatexFmt for DefinitionDerivation<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (mut env_start, mut env_end) = conf.mathenv_strs();
        if !conf.use_frac_array && conf.include_envs {
            env_start = "\\begin{prooftree}".to_owned();
            env_end = "\\end{prooftree}".to_owned();
        };

        conf.include_envs = false;
        let body_str = self.body_derivation.to_latex(conf);
        conf.include_envs = false;
        let ty_str = self.body_derivation.ty().to_latex(conf);

        if conf.use_frac_array {
            format!(
                "{env_start}\n\\frac{{ {body_str} }}{{ \\vdash {}:{} }}\n{env_end}",
                self.name, ty_str
            )
        } else {
            format!(
                "{env_start}\n{body_str}\n\\UnaryInfC{{$\\vdash {}:{}$}}\n{env_end}",
                self.name, ty_str
            )
        }
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
    let (env_start, env_end) = if conf.include_envs {
        ("\\begin{prooftree}", "\\end{prooftree}")
    } else {
        ("", "")
    };

    conf.include_envs = false;
    let mut conc_str = deriv.conc.to_latex(conf);
    conc_str = match deriv.premises.len() {
        0 => format!("\\UnaryInfC{{${conc_str}$}}",),
        1 => format!("\\UnaryInfC{{${conc_str}$}}",),
        2 => format!("\\BinaryInfC{{${conc_str}$}}",),
        3 => format!("\\TrinaryInfC{{${conc_str}$}}",),
        4 => format!("\\QuaternaryInfC{{${conc_str}$}}",),
        5 => format!("\\QuinaryInfC{{${conc_str}$}}",),
        _ => panic!("Derivations with more than 5 premises are not supported"),
    };

    let label_str = format!("\\RightLabel{{ {} }}\n", deriv.label);

    let start_str = if deriv.premises.is_empty() {
        "\\AxiomC{\\quad}\n"
    } else {
        ""
    };

    let mut prem_strs = vec![];
    for prem in deriv.premises.iter() {
        prem_strs.push(prem.to_latex(conf));
        conf.include_envs = false;
    }

    format!(
        "{env_start}{start_str}\n{}\n{label_str}\n{conc_str}\n{env_end}",
        prem_strs.join("\n")
    )
}

fn derivation_to_frac_array<T, Ty>(
    deriv: &TypingDerivation<T, Ty>,
    conf: &mut LatexConfig,
) -> String
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    let (env_start, env_end) = conf.mathenv_strs();
    let mut prem_strs = vec![];

    for prem in deriv.premises.iter() {
        conf.include_envs = false;
        prem_strs.push(format!("\\displaystyle {}", prem.to_latex(conf)));
    }
    if prem_strs.is_empty() {
        prem_strs.push("\\quad".to_owned());
    }

    let cs = (0..prem_strs.len()).map(|_| "c").collect::<Vec<&str>>();

    let array_start = format!("\\begin{{array}}{{ {} }}", cs.join(" "));
    let array_end = "\\end{array}\n";

    let array_str = format!("{array_start}\n{}\n{array_end}", prem_strs.join(" &\n "));

    conf.include_envs = false;
    let conc_str = deriv.conc.to_latex(conf);

    format!(
        "{env_start}\n\\frac{{\n{array_str}}}{{\n{conc_str}
        }}\n{env_end}"
    )
}
