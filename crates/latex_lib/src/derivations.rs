use super::{LatexConfig, LatexFmt};
use derivation::{DefinitionDerivation, ProgramDerivation, TypingDerivation};
use syntax::{terms::Term, types::Type};

impl<T, Ty> LatexFmt for ProgramDerivation<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        println!("formatting program with conf {conf:?}");
        println!(
            "definitions: {}",
            self.def_derivations
                .iter()
                .map(|def| def.name.clone())
                .collect::<Vec<String>>()
                .join(",")
        );
        let mut def_strs = vec![];
        let old_inc = conf.include_envs;
        for def in self.def_derivations.iter() {
            def_strs.push(def.to_latex(conf));
            conf.include_envs = old_inc;
        }

        println!("formatting program main");
        format!(
            " {}\\quad \\\\ \\quad \\\\{}",
            def_strs.join("\n\\quad \\\\ \\quad \\\\\n"),
            self.main_derivation.to_latex(conf),
        )
    }
}

impl<T, Ty> LatexFmt for DefinitionDerivation<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (mut env_start, mut env_end) = conf.mathenv_strs();
        if !conf.use_frac_array {
            env_start = "\\begin{prooftree}".to_owned();
            env_end = "\\end{prooftree}".to_owned();
        };

        conf.include_envs = false;
        let body_str = self.body_derivation.to_latex(conf);

        if conf.use_frac_array {
            format!(
                "{env_start}\n\\frac{{ {body_str} }}{{ \\vdash {}:{} }}\n{env_end}",
                self.name, body_str
            )
        } else {
            format!(
                "{env_start}\n{body_str}\n\\UnaryInfC{{$\\vdash {}:{}$}}\n{env_end}",
                self.name, body_str
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
    println!("formatting typing derivation buss {deriv:?}");
    let (env_start, env_end) = if conf.include_envs {
        ("\\begin{prooftree}", "\\end{prooftree}")
    } else {
        ("", "")
    };

    let conc_str = match deriv.premises.len() {
        0 => format!("\\UnaryInfC{{${}$}}", deriv.conc.to_latex(conf)),
        1 => format!("\\UnaryInfC{{${}$}}", deriv.conc.to_latex(conf)),
        2 => format!("\\BinaryInfC{{${}$}}", deriv.conc.to_latex(conf)),
        3 => format!("\\TrinaryInfC{{${}$}}", deriv.conc.to_latex(conf)),
        4 => format!("\\QuaternaryInfC{{${}$}}", deriv.conc.to_latex(conf)),
        5 => format!("\\QuinaryInfC{{${}$}}", deriv.conc.to_latex(conf)),
        _ => panic!("Derivations with more than 5 premises are not supported"),
    };

    let label_str = format!("\\RightLabel{{ {} }}\n", deriv.label);

    let start_str = if deriv.premises.is_empty() {
        "\\AxiomC{\\quad}\n"
    } else {
        ""
    };

    let mut prem_strs = vec![];
    let old_inc = conf.include_envs;
    for prem in deriv.premises.iter() {
        prem_strs.push(prem.to_latex(conf));
        conf.include_envs = old_inc;
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
