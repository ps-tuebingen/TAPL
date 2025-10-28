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
            Box::new(Symbol::Separated {
                fst: Box::new(Symbol::Typevariable),
                separator: Box::new(SpecialChar::LessColon.into()),
                snd: Box::new(Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(1.into()),
                }),
            })
        } else {
            Box::new(Symbol::Typevariable)
        };
        let ind_lower = if bounded { 1 } else { 2 };
        let ind_upper = if bounded { 2 } else { 3 };
        DerivationRule {
            premises: vec![ConclusionRule {
                env: Symbol::Separated {
                    fst: Box::new(SpecialChar::Gamma.into()),
                    separator: Box::new(SpecialChar::Comma.into()),
                    snd: var_sym.clone(),
                },
                input: Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(ind_lower.into()),
                },
                separator: SpecialChar::LessColon.into(),
                output: Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(ind_upper.into()),
                },
            }],
            label: format!("S-Exists{}", if bounded { "<" } else { "" }),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: Symbol::Prefixed {
                    prefix: Box::new(SpecialChar::Exists.into()),
                    inner: Box::new(Symbol::Separated {
                        fst: var_sym.clone(),
                        separator: Box::new(SpecialChar::Dot.into()),
                        snd: Box::new(Symbol::Subscript {
                            sym: Box::new(Symbol::Type),
                            ind: Box::new(ind_lower.into()),
                        }),
                    }),
                },
                separator: SpecialChar::LessColon.into(),
                output: Symbol::Prefixed {
                    prefix: Box::new(SpecialChar::Exists.into()),
                    inner: Box::new(Symbol::Separated {
                        fst: var_sym,
                        separator: Box::new(SpecialChar::Dot.into()),
                        snd: Box::new(Symbol::Subscript {
                            sym: Box::new(Symbol::Type),
                            ind: Box::new(ind_lower.into()),
                        }),
                    }),
                },
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
            Symbol::Separated {
                fst: Box::new(Symbol::Typevariable),
                separator: Box::new(SpecialChar::LessColon.into()),
                snd: Box::new(Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(1.into()),
                }),
            }
        } else {
            Symbol::Separated {
                fst: Box::new(Symbol::Typevariable),
                separator: Box::new(SpecialChar::DoubleColon.into()),
                snd: Box::new(Symbol::Kind),
            }
        };
        let prem_in = if bounded {
            Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(2.into()),
            }
        } else {
            Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(1.into()),
            }
        };
        let prem_out = if bounded {
            Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(3.into()),
            }
        } else {
            Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(2.into()),
            }
        };
        let premise = ConclusionRule {
            env: Symbol::Separated {
                fst: Box::new(SpecialChar::Gamma.into()),
                separator: Box::new(SpecialChar::Comma.into()),
                snd: Box::new(env_var),
            },
            input: prem_in,
            separator: SpecialChar::LessColon.into(),
            output: prem_out,
        };
        let conc_in = if bounded {
            Symbol::Prefixed {
                prefix: Box::new(SpecialChar::Forall.into()),
                inner: Box::new(Symbol::Separated {
                    fst: Box::new(Symbol::Separated {
                        fst: Box::new(Symbol::Typevariable),
                        separator: Box::new(SpecialChar::LessColon.into()),
                        snd: Box::new(Symbol::Subscript {
                            sym: Box::new(Symbol::Type),
                            ind: Box::new(1.into()),
                        }),
                    }),
                    separator: Box::new(SpecialChar::Dot.into()),
                    snd: Box::new(Symbol::Subscript {
                        sym: Box::new(Symbol::Type),
                        ind: Box::new(2.into()),
                    }),
                }),
            }
        } else {
            Symbol::Prefixed {
                prefix: Box::new(SpecialChar::Forall.into()),
                inner: Box::new(Symbol::Separated {
                    fst: Box::new(Symbol::Separated {
                        fst: Box::new(Symbol::Typevariable),
                        separator: Box::new(SpecialChar::DoubleColon.into()),
                        snd: Box::new(Symbol::Kind),
                    }),
                    separator: Box::new(SpecialChar::Dot.into()),
                    snd: Box::new(Symbol::Subscript {
                        sym: Box::new(Symbol::Type),
                        ind: Box::new(1.into()),
                    }),
                }),
            }
        };
        let conc_out = if bounded {
            Symbol::Prefixed {
                prefix: Box::new(SpecialChar::Forall.into()),
                inner: Box::new(Symbol::Separated {
                    fst: Box::new(Symbol::Separated {
                        fst: Box::new(Symbol::Typevariable),
                        separator: Box::new(SpecialChar::LessColon.into()),
                        snd: Box::new(Symbol::Subscript {
                            sym: Box::new(Symbol::Type),
                            ind: Box::new(1.into()),
                        }),
                    }),
                    separator: Box::new(SpecialChar::Dot.into()),
                    snd: Box::new(Symbol::Subscript {
                        sym: Box::new(Symbol::Type),
                        ind: Box::new(3.into()),
                    }),
                }),
            }
        } else {
            Symbol::Prefixed {
                prefix: Box::new(SpecialChar::Forall.into()),
                inner: Box::new(Symbol::Separated {
                    fst: Box::new(Symbol::Separated {
                        fst: Box::new(Symbol::Typevariable),
                        separator: Box::new(SpecialChar::DoubleColon.into()),
                        snd: Box::new(Symbol::Kind),
                    }),
                    separator: Box::new(SpecialChar::Dot.into()),
                    snd: Box::new(Symbol::Subscript {
                        sym: Box::new(Symbol::Type),
                        ind: Box::new(2.into()),
                    }),
                }),
            }
        };
        let conclusion = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: conc_in,
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
        let conc_in = Symbol::Separated {
            fst: Box::new(Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(1.into()),
            }),
            separator: Box::new(SpecialChar::Arrow.into()),
            snd: Box::new(Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(2.into()),
            }),
        };
        let conc_out = Symbol::Separated {
            fst: Box::new(Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(3.into()),
            }),
            separator: Box::new(SpecialChar::Arrow.into()),
            snd: Box::new(Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(4.into()),
            }),
        };
        let prem_from = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(3.into()),
            },
            separator: SpecialChar::LessColon.into(),
            output: Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(1.into()),
            },
        };
        let prem_to = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(2.into()),
            },
            separator: SpecialChar::LessColon.into(),
            output: Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(4.into()),
            },
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
            Symbol::Separated {
                fst: Box::new(Symbol::Typevariable),
                separator: Box::new(SpecialChar::LessColon.into()),
                snd: Box::new(Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(3.into()),
                }),
            }
        } else {
            Symbol::Separated {
                fst: Box::new(Symbol::Typevariable),
                separator: Box::new(SpecialChar::DoubleColon.into()),
                snd: Box::new(Symbol::Kind),
            }
        };

        let prem = ConclusionRule {
            env: Symbol::Separated {
                fst: Box::new(SpecialChar::Gamma.into()),
                separator: Box::new(SpecialChar::Comma.into()),
                snd: Box::new(annot.clone()),
            },
            input: Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(1.into()),
            },
            separator: SpecialChar::LessColon.into(),
            output: Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(2.into()),
            },
        };

        let conc_in = Symbol::Prefixed {
            prefix: Box::new(SpecialChar::Lambda.into()),
            inner: Box::new(Symbol::Separated {
                fst: Box::new(annot.clone()),
                separator: Box::new(SpecialChar::Dot.into()),
                snd: Box::new(Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(1.into()),
                }),
            }),
        };

        let conclusion = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: conc_in,
            separator: SpecialChar::LessColon.into(),
            output: Symbol::Prefixed {
                prefix: Box::new(SpecialChar::Lambda.into()),
                inner: Box::new(Symbol::Separated {
                    fst: Box::new(annot),
                    separator: Box::new(SpecialChar::Dot.into()),
                    snd: Box::new(Symbol::Subscript {
                        sym: Box::new(Symbol::Type),
                        ind: Box::new(2.into()),
                    }),
                }),
            },
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
                output: Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(1.into()),
                },
            }],
            label: "S-Rec".to_owned(),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: Symbol::Delim {
                    delim_open: SpecialChar::BrackO,
                    inner: Box::new(Symbol::Many(Box::new(Symbol::Separated {
                        fst: Box::new(Symbol::Label),
                        separator: Box::new(SpecialChar::Colon.into()),
                        snd: Box::new(Symbol::Subscript {
                            sym: Box::new(Symbol::Type),
                            ind: Box::new(1.into()),
                        }),
                    }))),
                    delim_close: SpecialChar::BrackC,
                },
                separator: SpecialChar::LessColon.into(),
                output: Symbol::Delim {
                    delim_open: SpecialChar::BrackO,
                    inner: Box::new(Symbol::Many(Box::new(Symbol::Separated {
                        fst: Box::new(Symbol::Label),
                        separator: Box::new(SpecialChar::Colon.into()),
                        snd: Box::new(Symbol::Subscript {
                            sym: Box::new(Symbol::Type),
                            ind: Box::new(2.into()),
                        }),
                    }))),
                    delim_close: SpecialChar::BrackC,
                },
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
                input: Symbol::Prefixed {
                    prefix: Box::new(Keyword::Ref.into()),
                    inner: Box::new(Symbol::Delim {
                        delim_open: SpecialChar::SqBrackO,
                        inner: Box::new(Symbol::Type),
                        delim_close: SpecialChar::SqBrackC,
                    }),
                },
                separator: SpecialChar::LessColon.into(),
                output: Symbol::Prefixed {
                    prefix: Box::new(Keyword::Sink.into()),
                    inner: Box::new(Symbol::Delim {
                        delim_open: SpecialChar::SqBrackO,
                        inner: Box::new(Symbol::Type),
                        delim_close: SpecialChar::SqBrackC,
                    }),
                },
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
                input: Symbol::Prefixed {
                    prefix: Box::new(Keyword::Ref.into()),
                    inner: Box::new(Symbol::Delim {
                        delim_open: SpecialChar::SqBrackO,
                        inner: Box::new(Symbol::Type),
                        delim_close: SpecialChar::SqBrackC,
                    }),
                },
                separator: SpecialChar::LessColon.into(),
                output: Symbol::Prefixed {
                    prefix: Box::new(Keyword::Source.into()),
                    inner: Box::new(Symbol::Delim {
                        delim_open: SpecialChar::SqBrackO,
                        inner: Box::new(Symbol::Type),
                        delim_close: SpecialChar::SqBrackC,
                    }),
                },
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
                input: Symbol::Separated {
                    fst: Box::new(Symbol::Typevariable),
                    separator: Box::new(SpecialChar::LessColon.into()),
                    snd: Box::new(Symbol::Type),
                },
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
                    input: Symbol::Subscript {
                        sym: Box::new(Symbol::Type),
                        ind: Box::new(Symbol::Separated {
                            fst: Box::new("i".into()),
                            separator: Box::new(SpecialChar::Comma.into()),
                            snd: Box::new("k".into()),
                        }),
                    },
                    separator: SpecialChar::LessColon.into(),
                    output: Symbol::Subscript {
                        sym: Box::new(Symbol::Type),
                        ind: Box::new(Symbol::Separated {
                            fst: Box::new("j".into()),
                            separator: Box::new(SpecialChar::Comma.into()),
                            snd: Box::new("k".into()),
                        }),
                    },
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
                input: Symbol::Delim {
                    delim_open: SpecialChar::AngBrackO,
                    inner: Box::new(Symbol::Many(Box::new(Symbol::Separated {
                        fst: Box::new(Symbol::Subscript {
                            sym: Box::new(Symbol::Label),
                            ind: Box::new("i".into()),
                        }),
                        separator: Box::new(SpecialChar::Colon.into()),
                        snd: Box::new(Symbol::Subscript {
                            sym: Box::new(Symbol::Type),
                            ind: Box::new(Symbol::Separated {
                                fst: Box::new("i".into()),
                                separator: Box::new(SpecialChar::Comma.into()),
                                snd: Box::new("k".into()),
                            }),
                        }),
                    }))),
                    delim_close: SpecialChar::AngBrackC,
                },
                separator: SpecialChar::LessColon.into(),
                output: Symbol::Delim {
                    delim_open: SpecialChar::AngBrackO,
                    inner: Box::new(Symbol::Many(Box::new(Symbol::Separated {
                        fst: Box::new(Symbol::Subscript {
                            sym: Box::new(Symbol::Label),
                            ind: Box::new("j".into()),
                        }),
                        separator: Box::new(SpecialChar::Colon.into()),
                        snd: Box::new(Symbol::Subscript {
                            sym: Box::new(Symbol::Type),
                            ind: Box::new(Symbol::Separated {
                                fst: Box::new("i".into()),
                                separator: Box::new(SpecialChar::Comma.into()),
                                snd: Box::new("k".into()),
                            }),
                        }),
                    }))),
                    delim_close: SpecialChar::AngBrackC,
                },
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
                input: Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(1.into()),
                },
                separator: SpecialChar::LessColon.into(),
                output: Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(2.into()),
                },
            }],
            label: "S-Cong".to_owned(),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: cong_fun(Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(1.into()),
                }),
                separator: SpecialChar::LessColon.into(),
                output: cong_fun(Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(2.into()),
                }),
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
            Symbol::Separated {
                fst: Box::new(Symbol::Typevariable),
                separator: Box::new(SpecialChar::LessColon.into()),
                snd: Box::new(Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(1.into()),
                }),
            }
        } else {
            Symbol::Separated {
                fst: Box::new(Symbol::Typevariable),
                separator: Box::new(SpecialChar::DoubleColon.into()),
                snd: Box::new(Symbol::Subscript {
                    sym: Box::new(Symbol::Kind),
                    ind: Box::new(1.into()),
                }),
            }
        };
        let prem_input = if bounded {
            Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(2.into()),
            }
        } else {
            Symbol::Type
        };
        let prem_output = if bounded {
            Symbol::Kind
        } else {
            Symbol::Subscript {
                sym: Box::new(Symbol::Kind),
                ind: Box::new(2.into()),
            }
        };
        let premise = ConclusionRule {
            env: Symbol::Separated {
                fst: Box::new(SpecialChar::Gamma.into()),
                separator: Box::new(SpecialChar::Comma.into()),
                snd: Box::new(prem_env_snd),
            },
            input: prem_input,
            separator: SpecialChar::DoubleColon.into(),
            output: prem_output,
        };
        let conc_input = if bounded {
            Symbol::Prefixed {
                prefix: Box::new(SpecialChar::Exists.into()),
                inner: Box::new(Symbol::Separated {
                    fst: Box::new(Symbol::Separated {
                        fst: Box::new(Symbol::Typevariable),
                        separator: Box::new(SpecialChar::LessColon.into()),
                        snd: Box::new(Symbol::Subscript {
                            sym: Box::new(Symbol::Type),
                            ind: Box::new(1.into()),
                        }),
                    }),
                    separator: Box::new(SpecialChar::Dot.into()),
                    snd: Box::new(Symbol::Subscript {
                        sym: Box::new(Symbol::Type),
                        ind: Box::new(2.into()),
                    }),
                }),
            }
        } else {
            Symbol::Prefixed {
                prefix: Box::new(SpecialChar::Exists.into()),
                inner: Box::new(Symbol::Separated {
                    fst: Box::new(Symbol::Separated {
                        fst: Box::new(Symbol::Typevariable),
                        separator: Box::new(SpecialChar::DoubleColon.into()),
                        snd: Box::new(Symbol::Subscript {
                            sym: Box::new(Symbol::Kind),
                            ind: Box::new(1.into()),
                        }),
                    }),
                    separator: Box::new(SpecialChar::Dot.into()),
                    snd: Box::new(Symbol::Type),
                }),
            }
        };
        let conc_output = if bounded {
            Symbol::Kind
        } else {
            Symbol::Subscript {
                sym: Box::new(Symbol::Kind),
                ind: Box::new(2.into()),
            }
        };
        let conc = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: conc_input,
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
            Symbol::Separated {
                fst: Box::new(Symbol::Typevariable),
                separator: Box::new(SpecialChar::LessColon.into()),
                snd: Box::new(Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(1.into()),
                }),
            }
        } else {
            Symbol::Separated {
                fst: Box::new(Symbol::Typevariable),
                separator: Box::new(SpecialChar::DoubleColon.into()),
                snd: Box::new(Symbol::Subscript {
                    sym: Box::new(Symbol::Kind),
                    ind: Box::new(1.into()),
                }),
            }
        };
        let prem_input = if bounded {
            Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(2.into()),
            }
        } else {
            Symbol::Type
        };
        let prem_output = if bounded {
            Symbol::Kind
        } else {
            Symbol::Subscript {
                sym: Box::new(Symbol::Kind),
                ind: Box::new(2.into()),
            }
        };
        let premise = ConclusionRule {
            env: Symbol::Separated {
                fst: Box::new(SpecialChar::Gamma.into()),
                separator: Box::new(SpecialChar::Comma.into()),
                snd: Box::new(tyvar),
            },
            input: prem_input,
            separator: SpecialChar::DoubleColon.into(),
            output: prem_output,
        };
        let conc_out = if bounded {
            Symbol::Kind
        } else {
            Symbol::Subscript {
                sym: Box::new(Symbol::Kind),
                ind: Box::new(2.into()),
            }
        };
        let conc_in_inner = if bounded {
            Symbol::Separated {
                fst: Box::new(Symbol::Separated {
                    fst: Box::new(Symbol::Typevariable),
                    separator: Box::new(SpecialChar::LessColon.into()),
                    snd: Box::new(Symbol::Subscript {
                        sym: Box::new(Symbol::Type),
                        ind: Box::new(1.into()),
                    }),
                }),
                separator: Box::new(SpecialChar::Dot.into()),
                snd: Box::new(Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(2.into()),
                }),
            }
        } else {
            Symbol::Separated {
                fst: Box::new(Symbol::Separated {
                    fst: Box::new(Symbol::Typevariable),
                    separator: Box::new(SpecialChar::DoubleColon.into()),
                    snd: Box::new(Symbol::Subscript {
                        sym: Box::new(Symbol::Kind),
                        ind: Box::new(1.into()),
                    }),
                }),
                separator: Box::new(SpecialChar::Dot.into()),
                snd: Box::new(Symbol::Type),
            }
        };
        let conc_in = Symbol::Prefixed {
            prefix: Box::new(SpecialChar::Forall.into()),
            inner: Box::new(conc_in_inner),
        };
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
            input: Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(1.into()),
            },
            separator: SpecialChar::DoubleColon.into(),
            output: SpecialChar::Star.into(),
        };
        let prem_to = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(2.into()),
            },
            separator: SpecialChar::DoubleColon.into(),
            output: SpecialChar::Star.into(),
        };
        let conclusion = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: Symbol::Separated {
                fst: Box::new(Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(1.into()),
                }),
                separator: Box::new(SpecialChar::Arrow.into()),
                snd: Box::new(Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(2.into()),
                }),
            },
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
            input: Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(1.into()),
            },
            separator: SpecialChar::DoubleColon.into(),
            output: Symbol::Separated {
                fst: Box::new(Symbol::Subscript {
                    sym: Box::new(Symbol::Kind),
                    ind: Box::new(2.into()),
                }),
                separator: Box::new(SpecialChar::Arrow.into()),
                snd: Box::new(Symbol::Subscript {
                    sym: Box::new(Symbol::Kind),
                    ind: Box::new(1.into()),
                }),
            },
        };
        let prem_to = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(2.into()),
            },
            separator: SpecialChar::DoubleColon.into(),
            output: Symbol::Subscript {
                sym: Box::new(Symbol::Kind),
                ind: Box::new(2.into()),
            },
        };
        let conclusion = ConclusionRule {
            env: SpecialChar::Gamma.into(),
            input: Symbol::Prefixed {
                prefix: Box::new(Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(1.into()),
                }),
                inner: Box::new(Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(2.into()),
                }),
            },
            separator: SpecialChar::DoubleColon.into(),
            output: Symbol::Subscript {
                sym: Box::new(Symbol::Kind),
                ind: Box::new(1.into()),
            },
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
            Box::new(Symbol::Separated {
                fst: Box::new(Symbol::Typevariable),
                separator: Box::new(SpecialChar::LessColon.into()),
                snd: Box::new(Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(1.into()),
                }),
            })
        } else {
            Box::new(Symbol::Separated {
                fst: Box::new(Symbol::Typevariable),
                separator: Box::new(SpecialChar::DoubleColon.into()),
                snd: Box::new(Symbol::Subscript {
                    sym: Box::new(Symbol::Kind),
                    ind: Box::new(3.into()),
                }),
            })
        };
        let prem_bound = vec![
            ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(1.into()),
                },
                separator: SpecialChar::DoubleColon.into(),
                output: Symbol::Subscript {
                    sym: Box::new(Symbol::Kind),
                    ind: Box::new(1.into()),
                },
            },
            ConclusionRule {
                env: Symbol::Separated {
                    fst: Box::new(SpecialChar::Gamma.into()),
                    separator: Box::new(SpecialChar::Comma.into()),
                    snd: annot.clone(),
                },
                input: Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(2.into()),
                },
                separator: SpecialChar::DoubleColon.into(),
                output: Symbol::Subscript {
                    sym: Box::new(Symbol::Kind),
                    ind: Box::new(2.into()),
                },
            },
        ];
        let prem_unbound = vec![ConclusionRule {
            env: Symbol::Separated {
                fst: Box::new(SpecialChar::Gamma.into()),
                separator: Box::new(SpecialChar::Comma.into()),
                snd: annot.clone(),
            },
            input: Symbol::Subscript {
                sym: Box::new(Symbol::Type),
                ind: Box::new(2.into()),
            },
            separator: SpecialChar::DoubleColon.into(),
            output: Symbol::Subscript {
                sym: Box::new(Symbol::Kind),
                ind: Box::new(2.into()),
            },
        }];
        DerivationRule {
            premises: if bounded { prem_bound } else { prem_unbound },
            label: format!("K-OpLam{}", if bounded { "<:" } else { "" }),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: Symbol::Prefixed {
                    prefix: Box::new(SpecialChar::Lambda.into()),
                    inner: Box::new(Symbol::Separated {
                        fst: annot,
                        separator: Box::new(SpecialChar::Dot.into()),
                        snd: Box::new(Symbol::Subscript {
                            sym: Box::new(Symbol::Type),
                            ind: Box::new(2.into()),
                        }),
                    }),
                },
                separator: SpecialChar::DoubleColon.into(),
                output: Symbol::Separated {
                    fst: Box::new(Symbol::Subscript {
                        sym: Box::new(Symbol::Kind),
                        ind: Box::new(1.into()),
                    }),
                    separator: Box::new(SpecialChar::Arrow.into()),
                    snd: Box::new(Symbol::Subscript {
                        sym: Box::new(Symbol::Kind),
                        ind: Box::new(2.into()),
                    }),
                },
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
                input: Symbol::Delim {
                    delim_open: SpecialChar::BrackO,
                    inner: Box::new(Symbol::Separated {
                        fst: Box::new(Symbol::Label),
                        separator: Box::new(SpecialChar::Colon.into()),
                        snd: Box::new(Symbol::Type),
                    }),
                    delim_close: SpecialChar::BrackC,
                },
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
                    input: Symbol::Subscript {
                        sym: Box::new(Symbol::Type),
                        ind: Box::new(1.into()),
                    },
                    separator: SpecialChar::DoubleColon.into(),
                    output: SpecialChar::Star.into(),
                },
                ConclusionRule {
                    env: SpecialChar::Gamma.into(),
                    input: Symbol::Subscript {
                        sym: Box::new(Symbol::Type),
                        ind: Box::new(2.into()),
                    },
                    separator: SpecialChar::DoubleColon.into(),
                    output: SpecialChar::Star.into(),
                },
            ],
            label: "K-Sum".to_owned(),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: Symbol::Separated {
                    fst: Box::new(Symbol::Subscript {
                        sym: Box::new(Symbol::Type),
                        ind: Box::new(1.into()),
                    }),
                    separator: Box::new(SpecialChar::Plus.into()),
                    snd: Box::new(Symbol::Subscript {
                        sym: Box::new(Symbol::Type),
                        ind: Box::new(2.into()),
                    }),
                },
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
                input: Symbol::Separated {
                    fst: Box::new(Symbol::Typevariable),
                    separator: Box::new(SpecialChar::DoubleColon.into()),
                    snd: Box::new(Symbol::Kind),
                },
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
                input: Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(1.into()),
                },
                separator: SpecialChar::Arrow.into(),
                output: Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(2.into()),
                },
            }],
            label: "Norm-Cong".to_owned(),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: conf_f(Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(1.into()),
                }),
                separator: SpecialChar::Arrow.into(),
                output: conf_f(Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(2.into()),
                }),
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
            Symbol::Separated {
                fst: Box::new(Symbol::Typevariable),
                separator: Box::new(SpecialChar::LessColon.into()),
                snd: Box::new(Symbol::Subscript {
                    sym: Box::new(Symbol::Type),
                    ind: Box::new(3.into()),
                }),
            }
        } else {
            Symbol::Separated {
                fst: Box::new(Symbol::Typevariable),
                separator: Box::new(SpecialChar::DoubleColon.into()),
                snd: Box::new(Symbol::Kind),
            }
        };
        DerivationRule {
            premises: vec![],
            label: format!("N-OpApp{}-Beta", if bounded { "<:" } else { "" }),
            conclusion: ConclusionRule {
                env: SpecialChar::Gamma.into(),
                input: Symbol::Separated {
                    fst: Box::new(Symbol::Prefixed {
                        prefix: Box::new(SpecialChar::Lambda.into()),
                        inner: Box::new(Symbol::Separated {
                            fst: Box::new(annot),
                            separator: Box::new(SpecialChar::Dot.into()),
                            snd: Box::new(Symbol::Subscript {
                                sym: Box::new(Symbol::Type),
                                ind: Box::new(1.into()),
                            }),
                        }),
                    }),
                    separator: Box::new(SpecialChar::Space.into()),
                    snd: Box::new(Symbol::Subscript {
                        sym: Box::new(Symbol::Type),
                        ind: Box::new(2.into()),
                    }),
                },
                separator: SpecialChar::Arrow.into(),
                output: Symbol::Prefixed {
                    prefix: Box::new(Symbol::Subscript {
                        sym: Box::new(Symbol::Type),
                        ind: Box::new(1.into()),
                    }),
                    inner: Box::new(Symbol::Delim {
                        delim_open: SpecialChar::SqBrackO,
                        inner: Box::new(Symbol::Separated {
                            fst: Box::new(Symbol::Typevariable),
                            separator: Box::new(SpecialChar::Arrow.into()),
                            snd: Box::new(Symbol::Subscript {
                                sym: Box::new(Symbol::Type),
                                ind: Box::new(2.into()),
                            }),
                        }),
                        delim_close: SpecialChar::SqBrackC,
                    }),
                },
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
