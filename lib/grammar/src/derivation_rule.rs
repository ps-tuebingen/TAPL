use crate::{
    Symbol,
    symbols::{Keyword, SpecialChar},
};

/// Rule for a typing derivation
/// For example
/// Gamma |-> t1:ty2->ty1   Gamma |-> t2:ty2
/// ---------------------------------------
///          Gamma |-> t1 t2: ty2
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DerivationRule {
    /// premises of the derivation rule
    pub premises: Vec<ConclusionRule>,
    /// Name of the rule
    pub label: String,
    /// Conclusion of the rule
    pub conclusion: ConclusionRule,
}

impl DerivationRule {
    /// Create a derivation rule for reflexive subtyping
    ///
    /// --------------------
    /// Gamma |-> Type<:Type
    pub fn sub_refl() -> DerivationRule {
        DerivationRule {
            premises: vec![],
            label: "S-Refl".to_owned(),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: Symbol::Type,
                separator: SpecialChar::LessColon.into(),
                output: Symbol::Type,
            },
        }
    }

    ///Derivation rule for subtyping of top
    ///
    /// ------------------
    /// Gamma |-> Type<:Top
    pub fn sub_top() -> DerivationRule {
        DerivationRule {
            premises: vec![],
            label: "S-Top".to_owned(),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: Symbol::Type,
                separator: SpecialChar::LessColon.into(),
                output: SpecialChar::Top.into(),
            },
        }
    }

    /// Derivation rule for subtyping of bot
    ///
    /// ---------------------
    /// Gamma |-> Bot <: Type
    pub fn sup_bot() -> DerivationRule {
        DerivationRule {
            premises: vec![],
            label: "S-Bot".to_owned(),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: SpecialChar::Bot.into(),
                separator: SpecialChar::LessColon.into(),
                output: Symbol::Type,
            },
        }
    }

    /// Derivation for subtyping existential types
    /// if bounded:
    /// Gamma, TypeVar<:Ty1 |-> Ty2 <: Ty3
    /// ------------------------------------
    /// Gamma |-> (exists TypeVar<:Ty1.Ty2) <: (exists TypeVar<:Ty1. Ty3)
    ///
    /// otherwise
    ///
    /// Gamma |-> Ty2 <: Ty3
    /// -------------------
    /// Gamma |-> exists TyperVar.Ty2 <: exists TypeVar.Ty3
    pub fn sub_exists(bounded: bool) -> DerivationRule {
        let var_sym = if bounded {
            vec![
                Symbol::Typevariable,
                SpecialChar::LessColon.into(),
                Symbol::sub(Symbol::Type, 1),
            ]
            .into()
        } else {
            Symbol::Typevariable
        };
        let ind_lower = if bounded { 1 } else { 2 };
        let ind_upper = if bounded { 2 } else { 3 };
        DerivationRule {
            premises: vec![ConclusionRule {
                env: vec![
                    SpecialChar::Gamma.into(),
                    SpecialChar::Comma.into(),
                    var_sym.clone(),
                ]
                .into(),
                input: Symbol::sub(Symbol::Type, ind_lower),
                separator: SpecialChar::LessColon.into(),
                output: Symbol::sub(Symbol::Type, ind_upper),
            }],
            label: format!("S-Exists{}", if bounded { "<" } else { "" }),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: vec![
                    SpecialChar::Exists.into(),
                    SpecialChar::Dot.into(),
                    Symbol::sub(Symbol::Type, ind_lower),
                ]
                .into(),
                separator: SpecialChar::LessColon.into(),
                output: vec![
                    SpecialChar::Exists.into(),
                    var_sym,
                    SpecialChar::Dot.into(),
                    Symbol::sub(Symbol::Type, ind_lower),
                ]
                .into(),
            },
        }
    }

    /// Derivation rule for subtyping universal types
    /// if bounded
    ///
    /// Gamma, TypeVar<:Ty1 |-> Ty2 <: Ty3
    /// ----------------------------------------------------
    /// Gamma |-> forall TypeVar<:Ty1.Ty2 <: forall TypeVar<:Ty1.Ty3
    ///
    /// otherwise
    ///
    /// Gamma,TypeVar::Kind |-> Ty1 <: Ty2
    /// -----------------------------------------------------------
    /// Gamma |-> forall TypeVar::Kind.Ty1 <: forall TypeVar::Kind.Ty2
    pub fn sub_forall(bounded: bool) -> DerivationRule {
        let env_var = if bounded {
            vec![
                Symbol::Typevariable,
                SpecialChar::LessColon.into(),
                Symbol::sub(Symbol::Type, 1),
            ]
            .into()
        } else {
            vec![
                Symbol::Typevariable,
                SpecialChar::DoubleColon.into(),
                Symbol::Kind,
            ]
            .into()
        };
        let prem_in = if bounded {
            Symbol::sub(Symbol::Type, 2)
        } else {
            Symbol::sub(Symbol::Type, 1)
        };
        let prem_out = if bounded {
            Symbol::sub(Symbol::Type, 3)
        } else {
            Symbol::sub(Symbol::Type, 2)
        };
        let premise = ConclusionRule {
            env: vec![
                SpecialChar::Gamma.into(),
                SpecialChar::Comma.into(),
                env_var,
            ]
            .into(),
            input: prem_in,
            separator: SpecialChar::LessColon.into(),
            output: prem_out,
        };
        let conc_in = if bounded {
            vec![
                SpecialChar::Forall.into(),
                Symbol::Typevariable,
                SpecialChar::LessColon.into(),
                Symbol::sub(Symbol::Type, 1),
                SpecialChar::Dot.into(),
                Symbol::sub(Symbol::Type, 2),
            ]
        } else {
            vec![
                SpecialChar::Forall.into(),
                Symbol::Typevariable,
                SpecialChar::DoubleColon.into(),
                Symbol::Kind,
                SpecialChar::Dot.into(),
                Symbol::sub(Symbol::Type, 1),
            ]
            .into()
        };
        let conc_out = if bounded {
            vec![
                SpecialChar::Forall.into(),
                Symbol::Typevariable,
                SpecialChar::LessColon.into(),
                Symbol::sub(Symbol::Type, 1),
                SpecialChar::Dot.into(),
                Symbol::sub(Symbol::Type, 3),
            ]
            .into()
        } else {
            vec![
                SpecialChar::Forall.into(),
                Symbol::Typevariable.into(),
                SpecialChar::DoubleColon.into(),
                Symbol::Kind,
                SpecialChar::Dot.into(),
                Symbol::sub(Symbol::Type, 2),
            ]
            .into()
        };
        let conclusion = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: conc_in.into(),
            separator: SpecialChar::LessColon.into(),
            output: conc_out,
        };
        DerivationRule {
            premises: vec![premise],
            label: format!("S-Forall{}", if bounded { "<:" } else { "" }),
            conclusion,
        }
    }

    /// Derivation rule for subtyping function types
    /// Gamma |-> Ty3 <: Ty1,  Gamma |-> Ty2 <: Ty4
    /// ------------------------------------------
    /// Gamma |-> Ty1 -> Ty2 <: Ty3 -> Ty4
    pub fn sub_fun() -> DerivationRule {
        let conc_in = vec![
            Symbol::sub(Symbol::Type, 1),
            SpecialChar::Arrow.into(),
            Symbol::sub(Symbol::Type, 2),
        ]
        .into();
        let conc_out = vec![
            Symbol::sub(Symbol::Type, 3),
            SpecialChar::Arrow.into(),
            Symbol::sub(Symbol::Type, 4),
        ]
        .into();
        let prem_from = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: Symbol::sub(Symbol::Type, 3),
            separator: SpecialChar::LessColon.into(),
            output: Symbol::sub(Symbol::Type, 1),
        };
        let prem_to = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: Symbol::sub(Symbol::Type, 2),
            separator: SpecialChar::LessColon.into(),
            output: Symbol::sub(Symbol::Type, 4),
        };
        DerivationRule {
            premises: vec![prem_from, prem_to],
            label: "S-Fun".to_owned(),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: conc_in,
                separator: SpecialChar::LessColon.into(),
                output: conc_out,
            },
        }
    }

    /// Derivation rule for subtyping op lambda
    /// if bounded
    ///
    /// Gamma,Typevar<:Ty3 |-> Ty1<:Ty2
    /// ---------------------------------------------
    /// Gamma |-> \TypeVar<:Ty3.Ty1 <: \TypeVar<:Ty3.Ty2
    ///
    /// otherwise
    /// Gamma,TypeVar::Kind |->  Ty1<:Ty2
    /// ----------------------------------------------
    /// Gamma |-> \Typevar::Kind.Ty1 <: \Typevar::Kind.Ty2
    pub fn sub_oplam(bounded: bool) -> DerivationRule {
        let annot = if bounded {
            vec![
                Symbol::Typevariable,
                SpecialChar::LessColon.into(),
                Symbol::sub(Symbol::Type, 3),
            ]
        } else {
            vec![
                Symbol::Typevariable,
                SpecialChar::DoubleColon.into(),
                Symbol::Kind,
            ]
        };

        let prem = ConclusionRule {
            env: vec![
                SpecialChar::Gamma.into(),
                SpecialChar::Comma.into(),
                annot.clone().into(),
            ]
            .into(),
            input: Symbol::sub(Symbol::Type, 1),
            separator: SpecialChar::LessColon.into(),
            output: Symbol::sub(Symbol::Type, 2),
        };

        let conc_in = vec![
            SpecialChar::Lambda.into(),
            annot.clone().into(),
            SpecialChar::Dot.into(),
            Symbol::sub(Symbol::Type, 1),
        ];

        let conclusion = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: conc_in.into(),
            separator: SpecialChar::LessColon.into(),
            output: vec![
                SpecialChar::Lambda.into(),
                annot.into(),
                SpecialChar::Dot.into(),
                Symbol::sub(Symbol::Type, 2),
            ]
            .into(),
        };
        DerivationRule {
            premises: vec![prem],
            label: "S-OpLam".to_owned(),
            conclusion,
        }
    }

    ///Derivation for subtyping record types
    /// Gamma |-> Term <: Type_1
    /// -----------------------
    /// Gamma |-> {label:Type_1} <: {label:Type_2}
    pub fn sub_rec() -> DerivationRule {
        DerivationRule {
            premises: vec![ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: Symbol::Term,
                separator: SpecialChar::LessColon.into(),
                output: Symbol::sub(Symbol::Type, 1),
            }],
            label: "S-Rec".to_owned(),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: vec![
                    SpecialChar::BrackO.into(),
                    Symbol::Label,
                    SpecialChar::Colon.into(),
                    Symbol::sub(Symbol::Type, 1),
                    SpecialChar::BrackC.into(),
                ]
                .into(),
                separator: SpecialChar::LessColon.into(),
                output: vec![
                    SpecialChar::BrackO.into(),
                    Symbol::Label,
                    SpecialChar::Colon.into(),
                    Symbol::sub(Symbol::Type, 2),
                    SpecialChar::BrackC.into(),
                ]
                .into(),
            },
        }
    }

    /// Derivation rule for subtyping references and sinks
    ///
    /// -----------------------------
    /// Gamma |-> Ref[Ty1] <: Sink[Ty1]
    pub fn sub_ref_sink() -> DerivationRule {
        DerivationRule {
            premises: vec![],
            label: "S-Ref-Sink".to_owned(),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: vec![
                    Keyword::Ref.into(),
                    SpecialChar::SqBrackO.into(),
                    Symbol::Type,
                    SpecialChar::SqBrackC.into(),
                ]
                .into(),
                separator: SpecialChar::LessColon.into(),
                output: vec![
                    Keyword::Sink.into(),
                    SpecialChar::SqBrackO.into(),
                    Symbol::Type,
                    SpecialChar::SqBrackC.into(),
                ]
                .into(),
            },
        }
    }

    /// Derivation rule for subtyping references and sources
    ///
    /// --------------------------------
    /// Gamma |-> Ref[Ty1] <: Source[Ty1]
    pub fn sub_ref_source() -> DerivationRule {
        DerivationRule {
            premises: vec![],
            label: "S-Ref-Source".to_owned(),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: vec![
                    Keyword::Ref.into(),
                    SpecialChar::SqBrackO.into(),
                    Symbol::Type,
                    SpecialChar::SqBrackC.into(),
                ]
                .into(),
                separator: SpecialChar::LessColon.into(),
                output: vec![
                    Keyword::Source.into(),
                    SpecialChar::SqBrackO.into(),
                    Symbol::Type,
                    SpecialChar::SqBrackC.into(),
                ]
                .into(),
            },
        }
    }

    /// Derivation Rule for subtyping variables
    /// TypeVariable <: Type in Gamma
    /// -----------------------------
    /// Gamma |-> Typevariable <: Type
    pub fn sub_var() -> DerivationRule {
        DerivationRule {
            premises: vec![ConclusionRule {
                env: SpecialChar::Space.into(),
                input: vec![
                    Symbol::Typevariable,
                    SpecialChar::LessColon.into(),
                    Symbol::Type,
                ]
                .into(),
                separator: Keyword::In.into(),
                output: SpecialChar::Gamma.into(),
            }],
            label: "S-Var".to_owned(),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: Symbol::Typevariable,
                separator: SpecialChar::LessColon.into(),
                output: Symbol::Type,
            },
        }
    }

    /// Derivation rule for subtyping variant types
    /// n <= m  Gamma |-> Type_ik <: Type_jk ...
    /// ----------------------------------------
    /// Gamma |-> <label_1:Type_i1,...label_n:Type_in> <: <label_1:Type_j1,...,label_jm>
    pub fn sub_variant() -> DerivationRule {
        DerivationRule {
            premises: vec![
                ConclusionRule {
                    env: SpecialChar::Space.into(),
                    input: "n".into(),
                    separator: SpecialChar::LessEq.into(),
                    output: "m".into(),
                },
                ConclusionRule {
                    env: SpecialChar::Gamma.into(),
                    input: Symbol::sub(
                        Symbol::Type,
                        vec!["i".into(), SpecialChar::Comma.into(), "k".into()],
                    ),
                    separator: SpecialChar::LessColon.into(),
                    output: Symbol::sub(
                        Symbol::Type,
                        vec!["j".into(), SpecialChar::Comma.into(), "k".into()],
                    ),
                },
                ConclusionRule {
                    env: SpecialChar::Space.into(),
                    input: SpecialChar::Ellipses.into(),
                    separator: SpecialChar::Space.into(),
                    output: SpecialChar::Space.into(),
                },
            ],
            label: "S-Variant".to_owned(),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: vec![
                    SpecialChar::AngBrackO.into(),
                    Symbol::many(vec![
                        Symbol::sub(Symbol::Label, "i"),
                        SpecialChar::Colon.into(),
                        Symbol::sub(
                            Symbol::Type,
                            vec!["i".into(), SpecialChar::Comma.into(), "k".into()],
                        ),
                    ]),
                    SpecialChar::AngBrackC.into(),
                ]
                .into(),

                separator: SpecialChar::LessColon.into(),
                output: vec![
                    SpecialChar::AngBrackO.into(),
                    Symbol::many(vec![
                        Symbol::sub(Symbol::Label, "j"),
                        SpecialChar::Colon.into(),
                        Symbol::sub(
                            Symbol::Type,
                            vec!["i".into(), SpecialChar::Comma.into(), "k".into()],
                        ),
                    ]),
                    SpecialChar::AngBrackC.into(),
                ]
                .into(),
            },
        }
    }

    /// Derivation rule for subtyping covariant congruence
    /// the given function constructs the congruent type
    pub fn sub_cong<F>(cong_fun: F) -> DerivationRule
    where
        F: Fn(Symbol) -> Symbol,
    {
        DerivationRule {
            premises: vec![ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: Symbol::sub(Symbol::Type, 1),
                separator: SpecialChar::LessColon.into(),
                output: Symbol::sub(Symbol::Type, 2),
            }],
            label: "S-Cong".to_owned(),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: cong_fun(Symbol::sub(Symbol::Type, 1)),
                separator: SpecialChar::LessColon.into(),
                output: cong_fun(Symbol::sub(Symbol::Type, 2)),
            },
        }
    }

    /// Derivation rule for primitive types with kind *
    /// with given symbol for the primitive type
    ///
    /// -----------------
    /// Gamma |-> sym :: *
    pub fn kind_prim(sym: Symbol) -> DerivationRule {
        DerivationRule {
            premises: vec![],
            label: "K-Prim".to_owned(),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: sym,
                separator: SpecialChar::DoubleColon.into(),
                output: SpecialChar::Star.into(),
            },
        }
    }

    /// Derivation Rule for types which can have any kind
    /// usually for types with annotated kinds
    ///
    /// -----------------
    /// Gamma |-> sym :: K
    pub fn kind_any(sym: Symbol) -> DerivationRule {
        DerivationRule {
            premises: vec![],
            label: "K-Any".to_owned(),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: sym,
                separator: SpecialChar::DoubleColon.into(),
                output: Symbol::Kind,
            },
        }
    }

    ///Derivation rule for kinding existential types
    ///if bounded
    /// Gamma, TypeVar<:Ty1 |-> Ty2::Kind
    /// ------------------------------------
    /// Gamma |-> exists TypeVar <: Ty1. Ty2 :: Kind
    ///
    /// otherwise
    ///
    /// Gamma,TypeVar::Kind1 |-> Type::Kind2
    /// ----------------------------------
    /// Gamma |-> exists TypeVar::Kind1.Type :: Kind2
    pub fn kind_exists(bounded: bool) -> DerivationRule {
        let prem_env_snd = if bounded {
            vec![
                Symbol::Typevariable,
                SpecialChar::LessColon.into(),
                Symbol::sub(Symbol::Type, 1),
            ]
        } else {
            vec![
                Symbol::Typevariable,
                SpecialChar::DoubleColon.into(),
                Symbol::sub(Symbol::Kind, 1),
            ]
        };
        let prem_input = if bounded {
            Symbol::sub(Symbol::Type, 2)
        } else {
            Symbol::Type
        };
        let prem_output = if bounded {
            Symbol::Kind
        } else {
            Symbol::sub(Symbol::Kind, 2)
        };
        let premise = ConclusionRule {
            env: vec![
                SpecialChar::Gamma.into(),
                SpecialChar::Comma.into(),
                prem_env_snd.into(),
            ]
            .into(),
            input: prem_input,
            separator: SpecialChar::DoubleColon.into(),
            output: prem_output,
        };
        let conc_input = if bounded {
            vec![
                SpecialChar::Exists.into(),
                Symbol::Typevariable,
                SpecialChar::LessColon.into(),
                Symbol::sub(Symbol::Type, 1),
                SpecialChar::Dot.into(),
                Symbol::sub(Symbol::Type, 2),
            ]
            .into()
        } else {
            vec![
                SpecialChar::Exists.into(),
                Symbol::Typevariable,
                SpecialChar::DoubleColon.into(),
                Symbol::sub(Symbol::Kind, 1),
                SpecialChar::Dot.into(),
                Symbol::Type,
            ]
        };
        let conc_output = if bounded {
            Symbol::Kind
        } else {
            Symbol::sub(Symbol::Kind, 2)
        };
        let conc = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: conc_input.into(),
            separator: SpecialChar::DoubleColon.into(),
            output: conc_output,
        };
        DerivationRule {
            premises: vec![premise],
            label: format!("K-Exists{}", if bounded { "<:" } else { "" }),
            conclusion: conc,
        }
    }

    /// Derivation Rule for kinding universal types
    /// if bounded
    /// Gamma, TypeVar<:Ty1 |-> Ty2 :: Kind
    /// ----------------------------------------
    /// Gamma |-> forall TypeVar<:Ty1.Ty2 :: Kind
    ///
    /// otherwise
    /// Gamma, TypeVar::Kind1 |-> Ty :: Kind2
    /// -----------------------------------------
    /// Gamma |-> forall TypeVar::Kind1. Ty :: Kind2
    pub fn kind_forall(bounded: bool) -> DerivationRule {
        let tyvar = if bounded {
            vec![
                Symbol::Typevariable,
                SpecialChar::LessColon.into(),
                Symbol::sub(Symbol::Type, 1),
            ]
            .into()
        } else {
            vec![
                Symbol::Typevariable,
                SpecialChar::DoubleColon.into(),
                Symbol::sub(Symbol::Kind, 1),
            ]
            .into()
        };
        let prem_input = if bounded {
            Symbol::sub(Symbol::Type, 2)
        } else {
            Symbol::Type
        };
        let prem_output = if bounded {
            Symbol::Kind
        } else {
            Symbol::sub(Symbol::Kind, 2)
        };
        let premise = ConclusionRule {
            env: vec![SpecialChar::Gamma.into(), SpecialChar::Comma.into(), tyvar].into(),
            input: prem_input,
            separator: SpecialChar::DoubleColon.into(),
            output: prem_output,
        };
        let conc_out = if bounded {
            Symbol::Kind
        } else {
            Symbol::sub(Symbol::Kind, 2)
        };
        let conc_in_inner = if bounded {
            vec![
                Symbol::Typevariable,
                SpecialChar::LessColon.into(),
                Symbol::sub(Symbol::Type, 1),
                SpecialChar::Dot.into(),
                Symbol::sub(Symbol::Type, 2),
            ]
            .into()
        } else {
            vec![
                Symbol::Typevariable,
                SpecialChar::DoubleColon.into(),
                Symbol::sub(Symbol::Kind, 1),
                SpecialChar::Dot.into(),
                Symbol::Type,
            ]
            .into()
        };
        let conc_in = vec![SpecialChar::Forall.into(), conc_in_inner].into();
        let conclusion = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: conc_in,
            separator: SpecialChar::DoubleColon.into(),
            output: conc_out,
        };

        DerivationRule {
            premises: vec![premise],
            label: format!("S-Forall{}", if bounded { "<:" } else { "" }),
            conclusion,
        }
    }

    /// Derivation rule for kinding function types
    ///
    /// Gamma |-> Ty1::*    Gamma|->Ty2::*
    /// -------------------------
    /// Gamma |-> Ty1 -> Ty2 :: *
    pub fn kind_fun() -> DerivationRule {
        let prem_from = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: Symbol::sub(Symbol::Type, 1),
            separator: SpecialChar::DoubleColon.into(),
            output: SpecialChar::Star.into(),
        };
        let prem_to = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: Symbol::sub(Symbol::Type, 2),
            separator: SpecialChar::DoubleColon.into(),
            output: SpecialChar::Star.into(),
        };
        let conclusion = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: vec![
                Symbol::sub(Symbol::Type, 1),
                SpecialChar::Arrow.into(),
                Symbol::sub(Symbol::Type, 2),
            ]
            .into(),

            separator: SpecialChar::DoubleColon.into(),
            output: SpecialChar::Star.into(),
        };
        DerivationRule {
            premises: vec![prem_from, prem_to],
            label: "K-Fun".to_owned(),
            conclusion,
        }
    }

    /// Derivation Rule for operator applications
    /// Gamma |-> Ty1 :: Kind2 -> Kind1 Gamma |-> Ty2::Kind2
    /// ---------------------------
    /// Gamma |-> Ty1 Ty2 :: Kind1
    pub fn kind_op_app() -> DerivationRule {
        let prem_from = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: Symbol::sub(Symbol::Type, 1),
            separator: SpecialChar::DoubleColon.into(),
            output: vec![
                Symbol::sub(Symbol::Kind, 2),
                SpecialChar::Arrow.into(),
                Symbol::sub(Symbol::Kind, 1),
            ]
            .into(),
        };
        let prem_to = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: Symbol::sub(Symbol::Type, 2),
            separator: SpecialChar::DoubleColon.into(),
            output: Symbol::sub(Symbol::Kind, 2),
        };
        let conclusion = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: vec![Symbol::sub(Symbol::Type, 1), Symbol::sub(Symbol::Type, 2)].into(),
            separator: SpecialChar::DoubleColon.into(),
            output: Symbol::sub(Symbol::Kind, 1),
        };
        DerivationRule {
            premises: vec![prem_from, prem_to],
            label: "K-OpApp".to_owned(),
            conclusion,
        }
    }

    /// Derivation Rule for kinding op lambda
    /// if bounded
    ///
    /// Gamma |-> Ty1::Kind1    Gamma,TypeVar<:Ty1 |-> Ty2 :: Kind2
    /// -------------------------------------------
    /// Gamma |-> \TypeVar<:Ty1.Ty2 :: Kind1 -> Kind2
    ///
    /// otherwise
    ///
    /// Gamma,TypeVar::Kind3 |-> Ty2 :: Kind2
    /// ---------------------------------------------
    /// Gamma |-> \TypeVar::Kind3.Ty2 :: Kind1 -> Kind2
    pub fn kind_op_lam(bounded: bool) -> DerivationRule {
        let annot = if bounded {
            vec![
                Symbol::Typevariable,
                SpecialChar::LessColon.into(),
                Symbol::sub(Symbol::Type, 1),
            ]
        } else {
            vec![
                Symbol::Typevariable,
                SpecialChar::DoubleColon.into(),
                Symbol::sub(Symbol::Kind, 3),
            ]
        };
        let mut prem_env = vec![SpecialChar::Gamma.into(), SpecialChar::Comma.into()];
        prem_env.extend(annot.clone());
        let prem_bound = vec![
            ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: Symbol::sub(Symbol::Type, 1),
                separator: SpecialChar::DoubleColon.into(),
                output: Symbol::sub(Symbol::Kind, 1),
            },
            ConclusionRule {
                env: prem_env.clone().into(),
                input: Symbol::sub(Symbol::Type, 2),
                separator: SpecialChar::DoubleColon.into(),
                output: Symbol::sub(Symbol::Kind, 2),
            },
        ];
        let prem_unbound = vec![ConclusionRule {
            env: prem_env.into(),
            input: Symbol::sub(Symbol::Type, 2),
            separator: SpecialChar::DoubleColon.into(),
            output: Symbol::sub(Symbol::Kind, 2),
        }];
        let mut conc_in = vec![SpecialChar::Lambda.into()];
        conc_in.extend(annot);
        conc_in.push(SpecialChar::Dot.into());
        conc_in.push(Symbol::sub(Symbol::Type, 2));

        DerivationRule {
            premises: if bounded { prem_bound } else { prem_unbound },
            label: format!("K-OpLam{}", if bounded { "<:" } else { "" }),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: conc_in.into(),
                separator: SpecialChar::DoubleColon.into(),
                output: vec![
                    Symbol::sub(Symbol::Kind, 1),
                    SpecialChar::Arrow.into(),
                    Symbol::sub(Symbol::Kind, 2),
                ]
                .into(),
            },
        }
    }

    /// Derivation Rule for kinding Record Types
    /// Gamma |-> Term :: *
    /// ------------------------------
    /// Gamma |-> { label : Term } :: *
    pub fn kind_rec() -> DerivationRule {
        DerivationRule {
            premises: vec![ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: Symbol::Term,
                separator: SpecialChar::DoubleColon.into(),
                output: SpecialChar::Star.into(),
            }],
            label: "K-Rec".to_owned(),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: vec![
                    SpecialChar::BrackO.into(),
                    Symbol::Label,
                    SpecialChar::Colon.into(),
                    Symbol::Type,
                    SpecialChar::BrackC.into(),
                ]
                .into(),
                separator: SpecialChar::DoubleColon.into(),
                output: SpecialChar::Star.into(),
            },
        }
    }

    /// Derivation Rule for kinding sum types
    /// Gamma |-> Type1 :: *    Gamma |-> Type2 :: *
    /// -------------------------------------
    /// Gamma |-> Type1 + Type2 :: *
    pub fn kind_sum() -> DerivationRule {
        DerivationRule {
            premises: vec![
                ConclusionRule {
                    env: SpecialChar::Gamma.into(),
                    input: Symbol::sub(Symbol::Type, 1),
                    separator: SpecialChar::DoubleColon.into(),
                    output: SpecialChar::Star.into(),
                },
                ConclusionRule {
                    env: SpecialChar::Gamma.into(),
                    input: Symbol::sub(Symbol::Type, 2),
                    separator: SpecialChar::DoubleColon.into(),
                    output: SpecialChar::Star.into(),
                },
            ],
            label: "K-Sum".to_owned(),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: vec![
                    Symbol::sub(Symbol::Type, 1),
                    SpecialChar::Plus.into(),
                    Symbol::sub(Symbol::Type, 2),
                ]
                .into(),

                separator: SpecialChar::DoubleColon.into(),
                output: SpecialChar::Star.into(),
            },
        }
    }

    /// Derivation Rule for kinding variables
    /// Typevariable :: Kind in Gamma
    /// -----------------------------
    /// Gamma |-> TypeVariable :: Kind
    pub fn kind_var() -> DerivationRule {
        DerivationRule {
            premises: vec![ConclusionRule {
                env: SpecialChar::Space.into(),
                input: vec![
                    Symbol::Typevariable,
                    SpecialChar::DoubleColon.into(),
                    Symbol::Kind,
                ]
                .into(),
                separator: Keyword::In.into(),
                output: SpecialChar::Gamma.into(),
            }],
            label: "K-Var".to_owned(),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: Symbol::Typevariable,
                separator: SpecialChar::DoubleColon.into(),
                output: Symbol::Kind,
            },
        }
    }

    /// Derivation Rule for normalizing congruence
    /// the given functions constructs a symbol representing the outer type
    pub fn norm_cong<F>(conf_f: F) -> DerivationRule
    where
        F: Fn(Symbol) -> Symbol,
    {
        DerivationRule {
            premises: vec![ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: Symbol::sub(Symbol::Type, 1),
                separator: SpecialChar::Arrow.into(),
                output: Symbol::sub(Symbol::Type, 2),
            }],
            label: "Norm-Cong".to_owned(),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: conf_f(Symbol::sub(Symbol::Type, 1)),
                separator: SpecialChar::Arrow.into(),
                output: conf_f(Symbol::sub(Symbol::Type, 2)),
            },
        }
    }

    ///Derivation Rule for operator application (beta reduction
    /// if bounded
    /// Gamma |-> (\Typevar<:Type3.Type1) Type2  -> Type1[TypeVar -> Type2]
    ///
    /// otherwise
    /// Gamma |-> (\Typevar::Kind.Type1) Type2 -> Type1[TypeVar -> Type2]
    pub fn norm_ap(bounded: bool) -> DerivationRule {
        let annot = if bounded {
            vec![
                Symbol::Typevariable,
                SpecialChar::LessColon.into(),
                Symbol::sub(Symbol::Type, 3),
            ]
            .into()
        } else {
            vec![
                Symbol::Typevariable,
                SpecialChar::DoubleColon.into(),
                Symbol::Kind,
            ]
            .into()
        };
        DerivationRule {
            premises: vec![],
            label: format!("N-OpApp{}-Beta", if bounded { "<:" } else { "" }),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: vec![
                    SpecialChar::Lambda.into(),
                    annot,
                    SpecialChar::Dot.into(),
                    Symbol::sub(Symbol::Type, 1),
                    Symbol::sub(Symbol::Type, 2),
                ]
                .into(),

                separator: SpecialChar::Arrow.into(),
                output: vec![
                    Symbol::sub(Symbol::Type, 1),
                    SpecialChar::SqBrackO.into(),
                    Symbol::Typevariable,
                    SpecialChar::Arrow.into(),
                    Symbol::sub(Symbol::Type, 2),
                    SpecialChar::SqBrackC.into(),
                ]
                .into(),
            },
        }
    }
}

/// Conclusion for a Derivation rule
/// for example Gamma |-> t:ty
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ConclusionRule {
    /// The symbol used for the environment
    /// usually Gamma
    pub env: Symbol,
    /// The symbol used in the left hand side of the rule (e.g. `t` for a term)
    pub input: Symbol,
    /// The separator symbol between input and output (e.g. `:` for a term)
    pub separator: Symbol,
    /// The symbol unsed in the right hand side of the rule (e.g. `ty` for a type)
    pub output: Symbol,
}

#[derive(Debug)]
pub struct LanguageRules {
    pub typing: Vec<DerivationRule>,
    pub subtyping: Vec<DerivationRule>,
    pub kinding: Vec<DerivationRule>,
    pub normalizing: Vec<DerivationRule>,
    pub eval: Vec<DerivationRule>,
}
