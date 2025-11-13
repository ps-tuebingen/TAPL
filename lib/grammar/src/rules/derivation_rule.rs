use crate::{
    ConclusionRule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use std::fmt;

/// Rule for a typing derivation
/// For example
/// ```text
/// Gamma |-> t1:ty2->ty1   Gamma |-> t2:ty2
/// ---------------------------------------
///          Gamma |-> t1 t2: ty2
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DerivationRule {
    /// premises of the derivation rule
    pub premises: Vec<ConclusionRule>,
    /// Name of the rule
    pub label: String,
    /// Conclusion of the rule
    pub conclusion: ConclusionRule,
}

impl DerivationRule {
    /// Derivation Rule for Typechecking Applications
    /// ```text
    /// Gamma |-> Term1 : Type1 -> Type2    Gamma |-> Term2: Type1
    /// ---------------------------------------------------------
    /// Gamma |-> Term1 Term2 : Type2
    /// ```
    #[must_use] pub fn check_ap() -> Self {
        let prem_fun = ConclusionRule::typing(
            Symbol::sub(Symbol::Term, 1),
            Symbol::arrow(Symbol::sub(Symbol::Type, 1), Symbol::sub(Symbol::Type, 2)),
        );
        let prem_arg =
            ConclusionRule::typing(Symbol::sub(Symbol::Term, 2), Symbol::sub(Symbol::Type, 1));
        Self {
            premises: vec![prem_fun, prem_arg],
            label: "T-Ap".to_owned(),
            conclusion: ConclusionRule::typing(
                vec![
                    Symbol::sub(Symbol::Term, 1),
                    SpecialChar::Space.into(),
                    Symbol::sub(Symbol::Term, 2),
                ],
                Symbol::sub(Symbol::Type, 2),
            ),
        }
    }

    /// Derivation Rule for checking let bindings
    /// ```text
    /// Gamma |-> Term1 : Ref[Type1]
    /// Gamma |-> Term2 : Type
    /// -----------------------------
    /// Gamma |-> Term1 := Term2 : Unit
    /// ```
    #[must_use] pub fn check_assign() -> Self {
        let prem_ref = ConclusionRule::typing(
            Symbol::sub(Symbol::Term, 1),
            vec![
                Keyword::Ref.into(),
                Symbol::sqbrack(Symbol::sub(Symbol::Type, 1)),
            ],
        );
        let prem_bound = ConclusionRule::typing(Symbol::sub(Symbol::Term, 2), Symbol::Type);
        Self {
            premises: vec![prem_ref, prem_bound],
            label: "T-Assign".to_owned(),
            conclusion: ConclusionRule::typing(
                vec![
                    Symbol::sub(Symbol::Term, 1),
                    SpecialChar::ColonEq.into(),
                    Symbol::sub(Symbol::Term, 2),
                ],
                Keyword::Unit,
            ),
        }
    }

    /// Derivation rule for checking Cons
    /// ```text
    /// Gamma |-> Term1:Type    Gamma |-> Term2: List[Type]
    /// -------------------------------------------------
    /// Gamma |-> Cons[Type](Term1,Term2) : List[Type]
    /// ```
    #[must_use] pub fn check_cons() -> Self {
        let conclusion = ConclusionRule::typing(
            vec![
                Keyword::Cons.into(),
                Symbol::sqbrack(Symbol::Type),
                Symbol::paren(Symbol::comma_sep(
                    Symbol::sub(Symbol::Term, 1),
                    Symbol::sub(Symbol::Term, 2),
                )),
            ],
            vec![Keyword::List.into(), Symbol::sqbrack(Symbol::Type)],
        );
        Self {
            premises: vec![
                ConclusionRule::typing(Symbol::sub(Symbol::Term, 1), Symbol::Type),
                ConclusionRule::typing(
                    Symbol::sub(Symbol::Term, 2),
                    vec![Keyword::List.into(), Symbol::sqbrack(Symbol::Type)],
                ),
            ],
            label: "T-Cons".to_owned(),
            conclusion,
        }
    }

    /// Derivation Rule for checking if expressions
    /// ```text
    /// Gamma |-> Term1 : Bool
    /// Gamma |-> Term2 : Type
    /// Gamma |-> Term3 : Type
    /// --------------------------------------
    /// Gamma |-> if Term1 { Term2 } else { Term3 } : Type
    /// ```
    #[must_use] pub fn check_if() -> Self {
        Self {
            premises: vec![
                ConclusionRule::typing(Symbol::sub(Symbol::Term, 1), Keyword::Bool),
                ConclusionRule::typing(Symbol::sub(Symbol::Term, 2), Symbol::Type),
                ConclusionRule::typing(Symbol::sub(Symbol::Term, 3), Symbol::Type),
            ],
            label: "T-If".to_owned(),
            conclusion: ConclusionRule::typing(
                vec![
                    Keyword::If.into(),
                    Symbol::sub(Symbol::Term, 1),
                    Symbol::brack(Symbol::sub(Symbol::Term, 2)),
                    Keyword::Else.into(),
                    Symbol::brack(Symbol::sub(Symbol::Term, 3)),
                ],
                Symbol::Type,
            ),
        }
    }

    /// Derivation Rule for checking lambda abstractions (either type or term)
    /// if bounded
    /// ```text
    /// Gamma, X <: Type1 |-> Term : Type2
    /// -------------------------------------------------------------
    /// Gamma |-> Lambda TypeVariable <: Type1. Term : Forall X <: Type1.Type2
    /// ```
    ///
    /// otherwise
    /// ```text
    /// Gamma,Variable:Type1 |-> Term:Type2
    /// ---------------------------------------------------
    /// Gamma |-> Lambda Variable:Type1.Term : Type1 -> Type2
    /// ```
    #[must_use] pub fn check_lambda(bounded: bool) -> Self {
        let prem_env = if bounded {
            Symbol::comma_sep(
                SpecialChar::Gamma,
                Symbol::less_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Type, 1)),
            )
        } else {
            Symbol::comma_sep(
                SpecialChar::Gamma,
                Symbol::colon_sep(Symbol::Variable, Symbol::sub(Symbol::Type, 1)),
            )
        };
        let conc_input = if bounded {
            vec![
                SpecialChar::Lambda.into(),
                Symbol::less_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Type, 1)),
                SpecialChar::Dot.into(),
                Symbol::Term,
            ]
        } else {
            vec![
                SpecialChar::Lambda.into(),
                Symbol::colon_sep(Symbol::Variable, Symbol::sub(Symbol::Type, 1)),
                SpecialChar::Dot.into(),
                Symbol::Term,
            ]
        };
        let conc_out = if bounded {
            vec![
                SpecialChar::Forall.into(),
                Symbol::less_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Type, 1)),
                SpecialChar::Dot.into(),
                Symbol::sub(Symbol::Type, 2),
            ]
            .into()
        } else {
            Symbol::arrow(Symbol::sub(Symbol::Type, 1), Symbol::sub(Symbol::Type, 2))
        };
        Self {
            premises: vec![
                ConclusionRule::typing(Symbol::Term, Symbol::sub(Symbol::Type, 2))
                    .with_env(prem_env),
            ],
            label: "T-Lambda".to_owned(),
            conclusion: ConclusionRule::typing(conc_input, conc_out),
        }
    }

    /// Derivation rule for checking let bindings
    /// ```text
    /// Gamma |-> Term1:Type1
    /// Gamma, x:Type1 |-> Term2:Type2
    /// -----------------------------------------------
    /// Gamma |-> let Variable = Term1 in Term2 : Type2
    /// ```
    #[must_use] pub fn check_let() -> Self {
        let prem_bound =
            ConclusionRule::typing(Symbol::sub(Symbol::Term, 1), Symbol::sub(Symbol::Term, 1));
        let prem_in =
            ConclusionRule::typing(Symbol::sub(Symbol::Term, 2), Symbol::sub(Symbol::Type, 2))
                .with_env(Symbol::comma_sep(
                    SpecialChar::Gamma,
                    Symbol::colon_sep(Symbol::Variable, Symbol::sub(Symbol::Type, 1)),
                ));
        Self {
            premises: vec![prem_bound, prem_in],
            label: "T-Let".to_owned(),
            conclusion: ConclusionRule::typing(
                vec![
                    Keyword::Let.into(),
                    Symbol::Variable,
                    SpecialChar::Equals.into(),
                    Symbol::sub(Symbol::Term, 1),
                    Keyword::Let.into(),
                    Symbol::sub(Symbol::Term, 2),
                ],
                Symbol::sub(Symbol::Term, 2),
            ),
        }
    }

    /// Derivation rule for checking anything that is looked up in the environment
    /// i.e. Locations and Variables
    /// ```text
    /// term:Type in Gamma
    /// -------------------
    /// Gamma |-> term: Type
    /// ```
    #[must_use] pub fn check_env(term: Symbol) -> Self {
        Self {
            premises: vec![ConclusionRule::lookup_env(vec![
                Symbol::colon_sep(term, Symbol::Type),
                Keyword::In.into(),
                SpecialChar::Gamma.into(),
            ])],
            label: "T-Loc".into(),
            conclusion: ConclusionRule::typing(Symbol::Location, Symbol::Type),
        }
    }

    /// Derivation Rule for checking pack terms
    /// if bounded
    /// ```text
    /// Gamma |-> Term : Type2[TypeVar -> Type1]
    /// Gamma |-> Type1 <: Type3
    /// ---------------------------------------
    /// Gamma |-> {*Type1,Term} as exists {TypeVar<:Type3,Type2}
    /// : exists {Typevar<:Type3,Type2}
    /// ```
    ///
    /// otherwise
    /// ```text
    /// Gamma |-> Term: Type2[Typevar -> Type1]
    /// ----------------------------------------------
    /// Gamma |-> {*Type1,Term} as exists {TypeVar::Kind,Type2} : exists {Typevar::Kind,Type2}
    /// ```
    #[must_use] pub fn check_pack(bounded: bool) -> Self {
        let ex_type = if bounded {
            vec![
                SpecialChar::Exists.into(),
                Symbol::brack(Symbol::less_colon_sep(
                    Symbol::Typevariable,
                    Symbol::comma_sep(Symbol::sub(Symbol::Type, 3), Symbol::sub(Symbol::Type, 2)),
                )),
            ]
        } else {
            vec![
                SpecialChar::Exists.into(),
                Symbol::brack(vec![Symbol::comma_sep(
                    Symbol::double_colon_sep(Symbol::Typevariable, Symbol::Kind),
                    Symbol::sub(Symbol::Type, 2),
                )]),
            ]
        };

        let prems_bounded = vec![
            ConclusionRule::typing(
                Symbol::Term,
                vec![
                    Symbol::sub(Symbol::Type, 2),
                    Symbol::sqbrack(Symbol::mapto(
                        Symbol::Typevariable,
                        Symbol::sub(Symbol::Type, 1),
                    )),
                ],
            ),
            ConclusionRule::typing(Symbol::sub(Symbol::Type, 1), Symbol::sub(Symbol::Type, 3)),
        ];
        let prems_unbounded = vec![ConclusionRule::typing(
            Symbol::Term,
            vec![
                Symbol::sub(Symbol::Type, 2),
                Symbol::sqbrack(Symbol::mapto(
                    Symbol::Typevariable,
                    Symbol::sub(Symbol::Type, 1),
                )),
            ],
        )];
        Self {
            premises: if bounded {
                prems_bounded
            } else {
                prems_unbounded
            },
            label: format!("T-Pack{}", if bounded { "<:" } else { "" }),
            conclusion: ConclusionRule::typing(
                vec![
                    Symbol::brack(vec![
                        SpecialChar::Star.into(),
                        Symbol::sub(Symbol::Type, 1),
                        Symbol::Term,
                    ]),
                    Keyword::As.into(),
                    ex_type.clone().into(),
                ],
                ex_type,
            ),
        }
    }

    /// Derivation Rule for checking unpack terms
    /// if bounded
    /// ```text
    /// Gamma |-> Term1 : Exists TypeVar<:Type3.Type2
    /// Gamma, TypeVar<:Type3,Var:Type2 |-> Term2: Type1
    /// -------------------------------------------------
    /// Gamma |-> Let {TypeVar,Var} = Term1 in Term2 : Type1
    /// ```
    ///
    /// otherwise
    /// ```text
    /// Gamma |-> Term1 : Exists TypeVar::Kind.Type2
    /// Gamma, TypeVar::Kind,Var:Type2 |-> Term2: Type1
    /// -------------------------------------------------
    /// Gamma |-> Let {TypeVar,Var} = Term1 in Term2 : Type1
    /// ```
    #[must_use] pub fn check_unpack(bounded: bool) -> Self {
        let ty_var = if bounded {
            Symbol::less_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Type, 3))
        } else {
            Symbol::double_colon_sep(Symbol::Typevariable, Symbol::Kind)
        };
        let prem_bound = ConclusionRule::typing(
            Symbol::sub(Symbol::Term, 1),
            vec![
                SpecialChar::Exists.into(),
                ty_var.clone(),
                SpecialChar::Dot.into(),
                Symbol::sub(Symbol::Type, 2),
            ],
        );
        let prem_in =
            ConclusionRule::typing(Symbol::sub(Symbol::Term, 2), Symbol::sub(Symbol::Type, 1))
                .with_env(Symbol::comma_sep(SpecialChar::Gamma, ty_var));
        Self {
            premises: vec![prem_bound, prem_in],
            label: format!("T-Unpack{}", if bounded { "<:" } else { "" }),
            conclusion: ConclusionRule::typing(
                vec![
                    Keyword::Let.into(),
                    Symbol::brack(Symbol::comma_sep(Symbol::Typevariable, Symbol::Variable)),
                    SpecialChar::Equals.into(),
                    Symbol::sub(Symbol::Term, 1),
                    Keyword::In.into(),
                    Symbol::sub(Symbol::Term, 2),
                ],
                Symbol::sub(Symbol::Type, 1),
            ),
        }
    }

    /// Derivation rule for checking pairs
    /// ```text
    /// Gamma |-> Term1 : Type1
    /// Gamma |-> Term2 : Type2
    /// ----------------------------------
    /// Gamma |-> {Term1,Term2} : Type1 x Type2
    /// ```
    #[must_use] pub fn check_pair() -> Self {
        Self {
            premises: vec![
                ConclusionRule::typing(Symbol::sub(Symbol::Term, 1), Symbol::sub(Symbol::Type, 2)),
                ConclusionRule::typing(Symbol::sub(Symbol::Term, 2), Symbol::sub(Symbol::Type, 2)),
            ],
            label: "T-Pair".to_owned(),
            conclusion: ConclusionRule::typing(
                Symbol::brack(Symbol::comma_sep(
                    Symbol::sub(Symbol::Term, 1),
                    Symbol::sub(Symbol::Term, 2),
                )),
                vec![
                    Symbol::sub(Symbol::Type, 1),
                    SpecialChar::Times.into(),
                    Symbol::sub(Symbol::Type, 2),
                ],
            ),
        }
    }

    /// Derivation rule for checking record terms
    /// ```text
    /// Gamma |-> Label_i : Type_i
    /// ----------------------------------
    /// Gamma |-> { Label_i = Term_i,... } : { Label_i : Type_i }
    /// ```
    #[must_use] pub fn check_record() -> Self {
        Self {
            premises: vec![ConclusionRule::typing(
                Symbol::sub(Symbol::Label, "i"),
                Symbol::sub(Symbol::Type, "i"),
            )],
            label: "T-Record".to_owned(),
            conclusion: ConclusionRule::typing(
                Symbol::brack(Symbol::many(vec![
                    Symbol::sub(Symbol::Label, "i"),
                    SpecialChar::Equals.into(),
                    Symbol::sub(Symbol::Term, "i"),
                ])),
                Symbol::brack(Symbol::many(vec![Symbol::colon_sep(
                    Symbol::sub(Symbol::Label, "i"),
                    Symbol::sub(Symbol::Type, "i"),
                )])),
            ),
        }
    }

    /// Derivation Rule for checking list case
    /// ```text
    /// Gamma |-> Term1: List[Type1]
    /// Gamma |-> Term2: Type2
    /// Gamma,Var1:Type1,Var2:List[Type2] |-> Term3: Type2
    /// ----------------------------------------------------------
    /// Gamma |-> case Term1 of { Nil => Term2 | Cons(Var1,Var2) => Term3 } : Type2
    /// ```
    #[must_use] pub fn check_listcase() -> Self {
        let prem_bound = ConclusionRule::typing(
            Symbol::sub(Symbol::Term, 1),
            vec![
                Keyword::List.into(),
                Symbol::sqbrack(Symbol::sub(Symbol::Type, 1)),
            ],
        );
        let prem_nil =
            ConclusionRule::typing(Symbol::sub(Symbol::Term, 2), Symbol::sub(Symbol::Type, 2));
        let prem_cons =
            ConclusionRule::typing(Symbol::sub(Symbol::Term, 2), Symbol::sub(Symbol::Type, 2))
                .with_env(Symbol::comma_sep(
                    Symbol::comma_sep(
                        SpecialChar::Gamma,
                        Symbol::colon_sep(
                            Symbol::sub(Symbol::Variable, 1),
                            Symbol::sub(Symbol::Type, 1),
                        ),
                    ),
                    Symbol::colon_sep(
                        Symbol::sub(Symbol::Variable, 2),
                        vec![
                            Keyword::List.into(),
                            Symbol::sqbrack(Symbol::sub(Symbol::Type, 2)),
                        ],
                    ),
                ));
        let conc = ConclusionRule::typing(
            vec![
                Keyword::Case.into(),
                Symbol::sub(Symbol::Term, 1),
                Keyword::Of.into(),
                Symbol::brack(vec![
                    Keyword::Nil.into(),
                    SpecialChar::DoubleArrow.into(),
                    Symbol::sub(Symbol::Term, 2),
                    SpecialChar::Pipe.into(),
                    Keyword::Cons.into(),
                    Symbol::paren(Symbol::comma_sep(
                        Symbol::sub(Symbol::Variable, 1),
                        Symbol::sub(Symbol::Variable, 2),
                    )),
                    SpecialChar::DoubleArrow.into(),
                    Symbol::sub(Symbol::Term, 3),
                ]),
            ],
            Symbol::sub(Symbol::Type, 2),
        );
        Self {
            premises: vec![prem_bound, prem_nil, prem_cons],
            label: "T-ListCase".to_owned(),
            conclusion: conc,
        }
    }
    /// Derivation rule for cases of something
    /// ```text
    /// Gamma |-> Term1 : Optional[Type1]
    /// Gamma |-> Term2 : Type2
    /// Gamma,Variable:Type1 |-> Term3 : Type2
    /// Gamma |-> case Term1 of { Nothing => Term2 | Something(Variable) => Term3 } : Type2
    /// ```
    #[must_use] pub fn check_somecase() -> Self {
        let prem_bound = ConclusionRule::typing(
            Symbol::sub(Symbol::Term, 1),
            vec![
                Keyword::Optional.into(),
                Symbol::sqbrack(Symbol::sub(Symbol::Type, 1)),
            ],
        );
        let prem_nothing =
            ConclusionRule::typing(Symbol::sub(Symbol::Term, 2), Symbol::sub(Symbol::Type, 2));
        let prem_something =
            ConclusionRule::typing(Symbol::sub(Symbol::Term, 3), Symbol::sub(Symbol::Type, 2))
                .with_env(Symbol::comma_sep(
                    SpecialChar::Gamma,
                    Symbol::colon_sep(Symbol::Variable, Symbol::sub(Symbol::Type, 1)),
                ));
        let conclusion = ConclusionRule::typing(
            vec![
                Keyword::Case.into(),
                Symbol::sub(Symbol::Term, 1),
                Keyword::Of.into(),
                Symbol::brack(vec![
                    Keyword::Nothing.into(),
                    SpecialChar::DoubleArrow.into(),
                    Symbol::sub(Symbol::Term, 2),
                    SpecialChar::Pipe.into(),
                    Keyword::Something.into(),
                    Symbol::paren(Symbol::Variable),
                    SpecialChar::DoubleArrow.into(),
                    Symbol::sub(Symbol::Term, 3),
                ]),
            ],
            Symbol::sub(Symbol::Type, 2),
        );
        Self {
            premises: vec![prem_bound, prem_nothing, prem_something],
            label: "T-SomeCase".to_owned(),
            conclusion,
        }
    }

    /// Derivation Rule for checking cases of sum types
    /// ```text
    /// Gamma |-> Term1: Type1 + Type2
    /// Gamma,Variable1:Type1 |-> Term2 : Type3
    /// Gamma, Variable2:Type2 |-> Term3: Type3
    /// ------------------------------------------
    /// Gamma |-> case Term1 of { inl(Variable1) => Term2 | inr(Variable2) => Term3 } : Type3
    /// ```
    #[must_use] pub fn check_sumcase() -> Self {
        let prem_bound = ConclusionRule::typing(
            Symbol::sub(Symbol::Term, 1),
            vec![
                Symbol::sub(Symbol::Type, 1),
                SpecialChar::Plus.into(),
                Symbol::sub(Symbol::Type, 2),
            ],
        );
        let prem_left =
            ConclusionRule::typing(Symbol::sub(Symbol::Term, 2), Symbol::sub(Symbol::Type, 3))
                .with_env(Symbol::comma_sep(
                    SpecialChar::Gamma,
                    Symbol::colon_sep(
                        Symbol::sub(Symbol::Variable, 1),
                        Symbol::sub(Symbol::Type, 1),
                    ),
                ));
        let prem_right =
            ConclusionRule::typing(Symbol::sub(Symbol::Term, 3), Symbol::sub(Symbol::Type, 3))
                .with_env(Symbol::comma_sep(
                    Symbol::comma_sep(SpecialChar::Gamma, Symbol::sub(Symbol::Variable, 2)),
                    Symbol::sub(Symbol::Type, 2),
                ));
        let conclusion = ConclusionRule::typing(
            vec![
                Keyword::Case.into(),
                Symbol::sub(Symbol::Term, 1),
                Keyword::Of.into(),
                Symbol::brack(vec![
                    Keyword::Left.into(),
                    Symbol::paren(Symbol::sub(Symbol::Variable, 1)),
                    SpecialChar::DoubleArrow.into(),
                    Symbol::sub(Symbol::Term, 2),
                    SpecialChar::Pipe.into(),
                    Keyword::Right.into(),
                    Symbol::paren(Symbol::sub(Symbol::Variable, 2)),
                    SpecialChar::DoubleArrow.into(),
                    Symbol::sub(Symbol::Term, 3),
                ]),
            ],
            Symbol::sub(Symbol::Type, 3),
        );
        Self {
            premises: vec![prem_bound, prem_left, prem_right],
            label: "T-SumCase".to_owned(),
            conclusion,
        }
    }

    ///Derivation rule for checking variant cases
    ///```text
    /// Gamma |-> Term : <Label_i:Term_i>
    /// Gamma |-> Term_k: Type
    /// ------------------------
    /// Gamma |-> case <Label_k=Term> of { Label_i => Term_i, ... } : Type
    /// ```
    #[must_use] pub fn check_variantcase() -> Self {
        let prem_bound = ConclusionRule::typing(
            Symbol::Term,
            Symbol::angbrack(Symbol::many(Symbol::colon_sep(
                Symbol::sub(Symbol::Label, "i"),
                Symbol::sub(Symbol::Term, "i"),
            ))),
        );
        let prem_rhs = ConclusionRule::typing(Symbol::sub(Symbol::Term, "k"), Symbol::Type);
        let conclusion = ConclusionRule::typing(
            vec![
                Keyword::Case.into(),
                Symbol::angbrack(vec![
                    Symbol::sub(Symbol::Label, "k"),
                    SpecialChar::Equals.into(),
                    Symbol::Term,
                ]),
                Keyword::Of.into(),
                Symbol::brack(Symbol::many(vec![
                    Symbol::sub(Symbol::Label, "i"),
                    SpecialChar::DoubleArrow.into(),
                    Symbol::sub(Symbol::Term, "i"),
                ])),
            ],
            Symbol::Type,
        );
        Self {
            premises: vec![prem_bound, prem_rhs],
            label: "T-VariantCase".to_owned(),
            conclusion,
        }
    }

    /// Derivation Rule for checking try terms (either with error or try catch
    /// with catch
    /// ```text
    ///Gamma |-> Term1: Type
    /// Gamma |-> Term2: Type2
    /// Gamma |-> try Term1 catch Term2 : Type
    /// ```
    ///otherwise
    ///```text
    /// Gamma |-> Term1: Type
    /// Gamma |-> Term2: Type
    /// Gamma |-> try Term1 with Term2 : Type
    /// ```
    #[must_use] pub fn check_tryt(catch: bool) -> Self {
        Self {
            premises: vec![
                ConclusionRule::typing(Symbol::sub(Symbol::Term, 1), Symbol::Type),
                ConclusionRule::typing(
                    Symbol::sub(Symbol::Term, 2),
                    if catch {
                        Symbol::sub(Symbol::Type, "exn")
                    } else {
                        Symbol::Type
                    },
                ),
            ],
            label: "T-Try".to_owned(),
            conclusion: ConclusionRule::typing(
                vec![
                    Keyword::Try.into(),
                    Symbol::sub(Symbol::Term, 1),
                    if catch {
                        Keyword::Catch.into()
                    } else {
                        Keyword::With.into()
                    },
                    Symbol::sub(Symbol::Term, 2),
                ],
                Symbol::Type,
            ),
        }
    }

    /// Derivation rule for checking tuples
    /// ```text
    /// Gamma |-> Term_i : Type_i
    /// ---------------------------
    /// Gamma |-> ( Term1,...) : (Type1,....)
    /// ```
    #[must_use] pub fn check_tuple() -> Self {
        Self {
            premises: vec![ConclusionRule::typing(
                Symbol::sub(Symbol::Term, "i"),
                Symbol::sub(Symbol::Type, "i"),
            )],
            label: "T-Tup".to_owned(),
            conclusion: ConclusionRule::typing(
                vec![Symbol::paren(Symbol::many(Symbol::sub(Symbol::Term, "i")))],
                vec![Symbol::paren(Symbol::many(Symbol::sub(Symbol::Type, "i")))],
            ),
        }
    }

    /// Derivation rule for checking type applications
    /// if bounded
    /// ```text
    /// Gamma |-> Term: forall TypeVar<:Type3.Type2
    /// Gamma |-> Type1 <: Type2
    /// ----------------------------------------------
    /// Gamma |-> Term [Type1] : Type2[TypeVar -> Type1]
    /// ```
    ///
    /// otherwise
    /// ```text
    /// Gamma |-> Term : forall TypeVar::Kind.Type2
    /// ------------------------------------------
    /// Gamma |-> Term [Type1] : Type2[TypeVar -> Type1]
    /// ```
    #[must_use] pub fn check_ty_app(bounded: bool) -> Self {
        let premises = if bounded {
            vec![
                ConclusionRule::typing(
                    Symbol::Term,
                    vec![
                        SpecialChar::Forall.into(),
                        Symbol::less_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Type, 3)),
                        SpecialChar::Dot.into(),
                        Symbol::sub(Symbol::Type, 2),
                    ],
                ),
                ConclusionRule::subtyping(
                    Symbol::sub(Symbol::Type, 1),
                    Symbol::sub(Symbol::Type, 2),
                ),
            ]
        } else {
            vec![ConclusionRule::typing(
                Symbol::Term,
                vec![
                    SpecialChar::Forall.into(),
                    Symbol::double_colon_sep(Symbol::Typevariable, Symbol::Kind),
                    SpecialChar::Dot.into(),
                    Symbol::sub(Symbol::Type, 2),
                ],
            )]
        };
        let conclusion = ConclusionRule::typing(
            vec![Symbol::Term, Symbol::sqbrack(Symbol::sub(Symbol::Type, 1))],
            vec![
                Symbol::sub(Symbol::Type, 2),
                Symbol::sqbrack(Symbol::mapto(
                    Symbol::Typevariable,
                    Symbol::sub(Symbol::Type, 1),
                )),
            ],
        );
        Self {
            premises,
            label: format!("T-TyApp{}", if bounded { "<:" } else { "" }),
            conclusion,
        }
    }

    /// derivation rule checking type abstractions
    /// if bounded
    /// ```text
    /// Gamma,TypeVar<:Type2 |-> Term: Type
    /// ------------------------------------
    /// Gamma |-> Lambda TypeVar <: Type2. Term : Forall TypeVar<:Type2.Type
    /// ```
    ///
    /// otherwise
    /// ```text
    /// Gamma, TypeVar::Kind |-> Term:Type
    /// --------------------------------------
    /// Gamma |-> Lambda TypeVar::Kind.Term : Forall TypeVar::Kind.Type
    /// ```
    #[must_use] pub fn check_ty_lambda(bounded: bool) -> Self {
        let prem_env = if bounded {
            Symbol::comma_sep(
                SpecialChar::Gamma,
                Symbol::less_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Type, 2)),
            )
        } else {
            Symbol::comma_sep(
                SpecialChar::Gamma,
                Symbol::double_colon_sep(Symbol::Typevariable, Symbol::Kind),
            )
        };
        let ty_var = if bounded {
            Symbol::less_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Type, 2))
        } else {
            Symbol::double_colon_sep(Symbol::Typevariable, Symbol::Kind)
        };
        Self {
            premises: vec![ConclusionRule::typing(Symbol::Term, Symbol::Type).with_env(prem_env)],
            label: format!("T-TyLam{}", if bounded { "<:" } else { "" }),
            conclusion: ConclusionRule::typing(
                vec![
                    SpecialChar::Lambda.into(),
                    ty_var.clone(),
                    SpecialChar::Dot.into(),
                    Symbol::Term,
                ],
                vec![
                    SpecialChar::Forall.into(),
                    ty_var,
                    SpecialChar::Dot.into(),
                    Symbol::Type,
                ],
            ),
        }
    }

    /// Derivation Rule for checking unfold terms
    /// ```text
    /// Gamma |-> Term : mu Typevariable.Type
    /// --------
    /// Gamma |-> unfold [mu Typevariable.Type] Term : Type[Typevariable -> Type]
    /// ```
    #[must_use] pub fn check_unfold() -> Self {
        let mu_ty = vec![
            SpecialChar::Mu.into(),
            Symbol::Typevariable,
            SpecialChar::Dot.into(),
            Symbol::Type,
        ];
        Self {
            premises: vec![ConclusionRule::typing(Symbol::Term, mu_ty.clone())],
            label: "T-Unfold".to_owned(),
            conclusion: ConclusionRule::typing(
                vec![Keyword::Unfold.into(), Symbol::sqbrack(mu_ty), Symbol::Term],
                vec![
                    Symbol::Type,
                    Symbol::sqbrack(Symbol::mapto(Symbol::Typevariable, Symbol::Type)),
                ],
            ),
        }
    }

    /// Derivation rule for checking terms with exactly one (simple) premise
    /// ```text
    /// Gamma |-> Term:ty_prem
    /// ------------------
    /// Gamma |-> term:ty_res
    /// ```
    pub fn check_cong<S1, S2, S3>(term: S1, ty_res: S2, ty_prem: S3, lb: &str) -> Self
    where
        S1: Into<Symbol>,
        S2: Into<Symbol>,
        S3: Into<Symbol>,
    {
        Self {
            premises: vec![ConclusionRule::typing(Symbol::Term, ty_prem.into())],
            label: lb.to_owned(),
            conclusion: ConclusionRule::typing(term, ty_res),
        }
    }

    /// Derivation rule for checking terms with no premises
    /// ```text
    ///
    /// ------------------
    /// Gamma |-> term : ty
    /// ```
    pub fn check_const<S1, S2>(term: S1, ty: S2, lb: &str) -> Self
    where
        S1: Into<Symbol>,
        S2: Into<Symbol>,
    {
        Self {
            premises: vec![],
            label: lb.to_owned(),
            conclusion: ConclusionRule::typing(term, ty),
        }
    }

    /// derivation rule for reflexive subtyping
    /// ```text
    ///
    /// --------------------
    /// Gamma |-> Type<:Type
    /// ```
    #[must_use] pub fn sub_refl() -> Self {
        Self {
            premises: vec![],
            label: "S-Refl".to_owned(),
            conclusion: ConclusionRule::subtyping(Symbol::Type, Symbol::Type),
        }
    }

    ///Derivation rule for subtyping of top
    ///```text
    ///
    /// ------------------
    /// Gamma |-> Type<:Top
    /// ```
    #[must_use] pub fn sub_top() -> Self {
        Self {
            premises: vec![],
            label: "S-Top".to_owned(),
            conclusion: ConclusionRule::subtyping(Symbol::Type, SpecialChar::Top),
        }
    }

    /// Derivation rule for subtyping of bot
    /// ```text
    ///
    /// ---------------------
    /// Gamma |-> Bot <: Type
    /// ```
    #[must_use] pub fn sup_bot() -> Self {
        Self {
            premises: vec![],
            label: "S-Bot".to_owned(),
            conclusion: ConclusionRule::subtyping(SpecialChar::Bot, Symbol::Type),
        }
    }

    /// Derivation for subtyping existential types
    /// if bounded:
    /// ```text
    /// Gamma, TypeVar<:Ty1 |-> Ty2 <: Ty3
    /// ------------------------------------
    /// Gamma |-> (exists TypeVar<:Ty1.Ty2) <: (exists TypeVar<:Ty1. Ty3)
    /// ```
    ///
    /// otherwise
    /// ```text
    /// Gamma |-> Ty2 <: Ty3
    /// -------------------
    /// Gamma |-> exists TyperVar.Ty2 <: exists TypeVar.Ty3
    /// ```
    #[must_use] pub fn sub_exists(bounded: bool) -> Self {
        let var_sym = if bounded {
            Symbol::less_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Type, 1))
        } else {
            Symbol::Typevariable
        };
        let ind_lower = if bounded { 1 } else { 2 };
        let ind_upper = if bounded { 2 } else { 3 };
        Self {
            premises: vec![
                ConclusionRule::subtyping(
                    Symbol::sub(Symbol::Type, ind_lower),
                    Symbol::sub(Symbol::Type, ind_upper),
                )
                .with_env(Symbol::comma_sep(SpecialChar::Gamma, var_sym.clone())),
            ],
            label: format!("S-Exists{}", if bounded { "<" } else { "" }),
            conclusion: ConclusionRule::subtyping(
                vec![
                    SpecialChar::Exists.into(),
                    SpecialChar::Dot.into(),
                    Symbol::sub(Symbol::Type, ind_lower),
                ],
                vec![
                    SpecialChar::Exists.into(),
                    var_sym,
                    SpecialChar::Dot.into(),
                    Symbol::sub(Symbol::Type, ind_lower),
                ],
            ),
        }
    }

    /// Derivation rule for subtyping universal types
    /// if bounded
    /// ```text
    /// Gamma, TypeVar<:Ty1 |-> Ty2 <: Ty3
    /// ----------------------------------------------------
    /// Gamma |-> forall TypeVar<:Ty1.Ty2 <: forall TypeVar<:Ty1.Ty3
    /// ```
    ///
    /// otherwise
    /// ```text
    /// Gamma,TypeVar::Kind |-> Ty1 <: Ty2
    /// -----------------------------------------------------------
    /// Gamma |-> forall TypeVar::Kind.Ty1 <: forall TypeVar::Kind.Ty2
    /// ```
    #[must_use] pub fn sub_forall(bounded: bool) -> Self {
        let env_var = if bounded {
            Symbol::less_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Type, 1))
        } else {
            Symbol::double_colon_sep(Symbol::Typevariable, Symbol::Kind)
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
        let premise = ConclusionRule::subtyping(prem_in, prem_out)
            .with_env(Symbol::comma_sep(SpecialChar::Gamma, env_var));
        let conc_in = if bounded {
            vec![
                SpecialChar::Forall.into(),
                Symbol::less_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Type, 1)),
                SpecialChar::Dot.into(),
                Symbol::sub(Symbol::Type, 2),
            ]
        } else {
            vec![
                SpecialChar::Forall.into(),
                Symbol::double_colon_sep(Symbol::Typevariable, Symbol::Kind),
                SpecialChar::Dot.into(),
                Symbol::sub(Symbol::Type, 1),
            ]
        };
        let conc_out = if bounded {
            vec![
                SpecialChar::Forall.into(),
                Symbol::less_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Type, 1)),
                SpecialChar::Dot.into(),
                Symbol::sub(Symbol::Type, 3),
            ]
        } else {
            vec![
                SpecialChar::Forall.into(),
                Symbol::double_colon_sep(Symbol::Typevariable, Symbol::Kind),
                SpecialChar::Dot.into(),
                Symbol::sub(Symbol::Type, 2),
            ]
        };
        let conclusion = ConclusionRule::subtyping(conc_in, conc_out);
        Self {
            premises: vec![premise],
            label: format!("S-Forall{}", if bounded { "<:" } else { "" }),
            conclusion,
        }
    }

    /// Derivation rule for subtyping function types
    /// ```text
    /// Gamma |-> Ty3 <: Ty1,  Gamma |-> Ty2 <: Ty4
    /// ------------------------------------------
    /// Gamma |-> Ty1 -> Ty2 <: Ty3 -> Ty4
    /// ```
    #[must_use] pub fn sub_fun() -> Self {
        let conc_in = Symbol::arrow(Symbol::sub(Symbol::Type, 1), Symbol::sub(Symbol::Type, 2));
        let conc_out = Symbol::arrow(Symbol::sub(Symbol::Type, 3), Symbol::sub(Symbol::Type, 4));
        let prem_from =
            ConclusionRule::subtyping(Symbol::sub(Symbol::Type, 3), Symbol::sub(Symbol::Type, 1));
        let prem_to =
            ConclusionRule::subtyping(Symbol::sub(Symbol::Type, 2), Symbol::sub(Symbol::Type, 4));
        Self {
            premises: vec![prem_from, prem_to],
            label: "S-Fun".to_owned(),
            conclusion: ConclusionRule::subtyping(conc_in, conc_out),
        }
    }

    /// Derivation rule for subtyping op lambda
    /// if bounded
    /// ```text
    /// Gamma,Typevar<:Ty3 |-> Ty1<:Ty2
    /// ---------------------------------------------
    /// Gamma |-> \TypeVar<:Ty3.Ty1 <: \TypeVar<:Ty3.Ty2
    /// ```
    ///
    /// otherwise
    /// ```text
    /// Gamma,TypeVar::Kind |->  Ty1<:Ty2
    /// ----------------------------------------------
    /// Gamma |-> \Typevar::Kind.Ty1 <: \Typevar::Kind.Ty2
    /// ```
    #[must_use] pub fn sub_oplam(bounded: bool) -> Self {
        let annot = if bounded {
            Symbol::less_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Type, 3))
        } else {
            Symbol::double_colon_sep(Symbol::Typevariable, Symbol::Kind)
        };

        let prem =
            ConclusionRule::subtyping(Symbol::sub(Symbol::Type, 1), Symbol::sub(Symbol::Type, 2))
                .with_env(Symbol::comma_sep(SpecialChar::Gamma, annot.clone()));

        let conc_in = vec![
            SpecialChar::Lambda.into(),
            annot.clone(),
            SpecialChar::Dot.into(),
            Symbol::sub(Symbol::Type, 1),
        ];

        let conclusion = ConclusionRule::subtyping(
            conc_in,
            vec![
                SpecialChar::Lambda.into(),
                annot,
                SpecialChar::Dot.into(),
                Symbol::sub(Symbol::Type, 2),
            ],
        );
        Self {
            premises: vec![prem],
            label: "S-OpLam".to_owned(),
            conclusion,
        }
    }

    ///Derivation for subtyping record types
    /// ```text
    /// Gamma |-> Term <: Type_1
    /// -----------------------
    /// Gamma |-> {label:Type_1} <: {label:Type_2}
    /// ```
    #[must_use] pub fn sub_rec() -> Self {
        Self {
            premises: vec![ConclusionRule::subtyping(
                Symbol::Term,
                Symbol::sub(Symbol::Type, 1),
            )],
            label: "S-Rec".to_owned(),
            conclusion: ConclusionRule::subtyping(
                Symbol::brack(Symbol::colon_sep(
                    Symbol::Label,
                    Symbol::sub(Symbol::Type, 1),
                )),
                Symbol::brack(Symbol::colon_sep(
                    Symbol::Label,
                    Symbol::sub(Symbol::Type, 2),
                )),
            ),
        }
    }

    /// Derivation rule for subtyping references and sinks
    /// ```text
    ///
    /// -----------------------------
    /// Gamma |-> Ref[Ty1] <: Sink[Ty1]
    /// ```
    #[must_use] pub fn sub_ref_sink() -> Self {
        Self {
            premises: vec![],
            label: "S-Ref-Sink".to_owned(),
            conclusion: ConclusionRule::subtyping(
                vec![Keyword::Ref.into(), Symbol::sqbrack(Symbol::Type)],
                vec![Keyword::Sink.into(), Symbol::sqbrack(Symbol::Type)],
            ),
        }
    }

    /// Derivation rule for subtyping references and sources
    /// ```text
    ///
    /// --------------------------------
    /// Gamma |-> Ref[Ty1] <: Source[Ty1]
    /// ```
    #[must_use] pub fn sub_ref_source() -> Self {
        Self {
            premises: vec![],
            label: "S-Ref-Source".to_owned(),
            conclusion: ConclusionRule::subtyping(
                vec![Keyword::Ref.into(), Symbol::sqbrack(Symbol::Type)],
                vec![Keyword::Source.into(), Symbol::sqbrack(Symbol::Type)],
            ),
        }
    }

    /// Derivation Rule for subtyping variables
    /// ```text
    /// TypeVariable <: Type in Gamma
    /// -----------------------------
    /// Gamma |-> Typevariable <: Type
    /// ```
    #[must_use] pub fn sub_var() -> Self {
        Self {
            premises: vec![ConclusionRule::lookup_env(Symbol::less_colon_sep(
                Symbol::Typevariable,
                Symbol::Type,
            ))],
            label: "S-Var".to_owned(),
            conclusion: ConclusionRule::subtyping(Symbol::Typevariable, Symbol::Type),
        }
    }

    /// Derivation rule for subtyping variant types
    /// ```text
    /// n <= m  Gamma |-> Type_ik <: Type_jk ...
    /// ----------------------------------------
    /// Gamma |-> <label_1:Type_i1,...label_n:Type_in> <: <label_1:Type_j1,...,label_jm>
    /// ```
    #[must_use] pub fn sub_variant() -> Self {
        Self {
            premises: vec![
                ConclusionRule::new(SpecialChar::Empty, "n", SpecialChar::LessEq, "m"),
                ConclusionRule::subtyping(
                    Symbol::sub(Symbol::Type, Symbol::comma_sep("i", "k")),
                    Symbol::sub(Symbol::Type, Symbol::comma_sep("j", "k")),
                ),
            ],
            label: "S-Variant".to_owned(),
            conclusion: ConclusionRule::subtyping(
                Symbol::angbrack(Symbol::many(Symbol::colon_sep(
                    Symbol::sub(Symbol::Label, "i"),
                    Symbol::sub(Symbol::Type, Symbol::comma_sep("i", "k")),
                ))),
                Symbol::angbrack(Symbol::many(Symbol::colon_sep(
                    Symbol::sub(Symbol::Label, "j"),
                    Symbol::sub(Symbol::Type, Symbol::comma_sep("i", "k")),
                ))),
            ),
        }
    }

    /// Derivation rule for subtyping covariant congruence
    /// the given function constructs the congruent type
    pub fn sub_cong<F>(cong_fun: F) -> Self
    where
        F: Fn(Symbol) -> Symbol,
    {
        Self {
            premises: vec![ConclusionRule::subtyping(
                Symbol::sub(Symbol::Type, 1),
                Symbol::sub(Symbol::Type, 2),
            )],
            label: "S-Cong".to_owned(),
            conclusion: ConclusionRule::subtyping(
                cong_fun(Symbol::sub(Symbol::Type, 1)),
                cong_fun(Symbol::sub(Symbol::Type, 2)),
            ),
        }
    }

    /// Derivation rule for primitive types with kind *
    /// with given symbol for the primitive type
    ///
    ///```text
    /// -----------------
    /// Gamma |-> sym :: *
    /// ```
    #[must_use] pub fn kind_prim(sym: Symbol) -> Self {
        Self {
            premises: vec![],
            label: "K-Prim".to_owned(),
            conclusion: ConclusionRule::kinding(sym, SpecialChar::Star),
        }
    }

    /// Derivation Rule for types which can have any kind
    /// usually for types with annotated kinds
    /// ```text
    ///
    /// -----------------
    /// Gamma |-> sym :: K
    /// ```
    #[must_use] pub fn kind_any(sym: Symbol) -> Self {
        Self {
            premises: vec![],
            label: "K-Any".to_owned(),
            conclusion: ConclusionRule::kinding(sym, Symbol::Kind),
        }
    }

    ///Derivation rule for kinding existential types
    ///if bounded
    ///```text
    /// Gamma, TypeVar<:Ty1 |-> Ty2::Kind
    /// ------------------------------------
    /// Gamma |-> exists TypeVar <: Ty1. Ty2 :: Kind
    /// ```
    ///
    /// otherwise
    /// ```text
    /// Gamma,TypeVar::Kind1 |-> Type::Kind2
    /// ----------------------------------
    /// Gamma |-> exists TypeVar::Kind1.Type :: Kind2
    /// ```
    #[must_use] pub fn kind_exists(bounded: bool) -> Self {
        let prem_env_snd = if bounded {
            Symbol::less_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Type, 1))
        } else {
            Symbol::double_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Kind, 1))
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
        let premise = ConclusionRule::kinding(prem_input, prem_output)
            .with_env(Symbol::comma_sep(SpecialChar::Gamma, prem_env_snd));
        let conc_input = if bounded {
            vec![
                SpecialChar::Exists.into(),
                Symbol::less_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Type, 1)),
                SpecialChar::Dot.into(),
                Symbol::sub(Symbol::Type, 2),
            ]
        } else {
            vec![
                SpecialChar::Exists.into(),
                Symbol::double_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Kind, 1)),
                SpecialChar::Dot.into(),
                Symbol::Type,
            ]
        };
        let conc_output = if bounded {
            Symbol::Kind
        } else {
            Symbol::sub(Symbol::Kind, 2)
        };
        let conc = ConclusionRule::kinding(conc_input, conc_output);
        Self {
            premises: vec![premise],
            label: format!("K-Exists{}", if bounded { "<:" } else { "" }),
            conclusion: conc,
        }
    }

    /// Derivation Rule for kinding universal types
    /// if bounded
    /// ```text
    /// Gamma, TypeVar<:Ty1 |-> Ty2 :: Kind
    /// ----------------------------------------
    /// Gamma |-> forall TypeVar<:Ty1.Ty2 :: Kind
    /// ```
    ///
    /// otherwise
    /// ```text
    /// Gamma, TypeVar::Kind1 |-> Ty :: Kind2
    /// -----------------------------------------
    /// Gamma |-> forall TypeVar::Kind1. Ty :: Kind2
    /// ```
    #[must_use] pub fn kind_forall(bounded: bool) -> Self {
        let tyvar = if bounded {
            Symbol::less_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Type, 1))
        } else {
            Symbol::double_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Kind, 1))
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
        let premise = ConclusionRule::kinding(prem_input, prem_output)
            .with_env(Symbol::comma_sep(SpecialChar::Gamma, tyvar));
        let conc_out = if bounded {
            Symbol::Kind
        } else {
            Symbol::sub(Symbol::Kind, 2)
        };
        let conc_in_inner = if bounded {
            vec![
                Symbol::less_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Type, 1)),
                SpecialChar::Dot.into(),
                Symbol::sub(Symbol::Type, 2),
            ]
        } else {
            vec![
                Symbol::double_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Kind, 1)),
                SpecialChar::Dot.into(),
                Symbol::Type,
            ]
        };
        let conc_in = vec![SpecialChar::Forall.into(), conc_in_inner.into()];
        let conclusion = ConclusionRule::kinding(conc_in, conc_out);

        Self {
            premises: vec![premise],
            label: format!("S-Forall{}", if bounded { "<:" } else { "" }),
            conclusion,
        }
    }

    /// Derivation rule for kinding function types
    ///
    /// ```text
    /// Gamma |-> Ty1::*    Gamma|->Ty2::*
    /// -------------------------
    /// Gamma |-> Ty1 -> Ty2 :: *
    /// ```
    #[must_use] pub fn kind_fun() -> Self {
        let prem_from = ConclusionRule::kinding(Symbol::sub(Symbol::Type, 1), SpecialChar::Star);
        let prem_to = ConclusionRule::kinding(Symbol::sub(Symbol::Type, 2), SpecialChar::Star);
        let conclusion = ConclusionRule::kinding(
            Symbol::arrow(Symbol::sub(Symbol::Type, 1), Symbol::sub(Symbol::Type, 2)),
            SpecialChar::Star,
        );
        Self {
            premises: vec![prem_from, prem_to],
            label: "K-Fun".to_owned(),
            conclusion,
        }
    }

    /// Derivation Rule for operator applications
    /// ```text
    /// Gamma |-> Ty1 :: Kind2 => Kind1
    /// Gamma |-> Ty2::Kind2
    /// ---------------------------
    /// Gamma |-> Ty1 Ty2 :: Kind1
    /// ```
    #[must_use] pub fn kind_op_app() -> Self {
        let prem_from = ConclusionRule::kinding(
            Symbol::sub(Symbol::Type, 1),
            vec![
                Symbol::sub(Symbol::Kind, 2),
                SpecialChar::DoubleArrow.into(),
                Symbol::sub(Symbol::Kind, 1),
            ],
        );
        let prem_to =
            ConclusionRule::kinding(Symbol::sub(Symbol::Type, 2), Symbol::sub(Symbol::Kind, 2));
        let conclusion = ConclusionRule::kinding(
            vec![Symbol::sub(Symbol::Type, 1), Symbol::sub(Symbol::Type, 2)],
            Symbol::sub(Symbol::Kind, 1),
        );
        Self {
            premises: vec![prem_from, prem_to],
            label: "K-OpApp".to_owned(),
            conclusion,
        }
    }

    /// Derivation Rule for kinding op lambda
    /// if bounded
    /// ```text
    /// Gamma |-> Ty1::Kind1    Gamma,TypeVar<:Ty1 |-> Ty2 :: Kind2
    /// -------------------------------------------
    /// Gamma |-> \TypeVar<:Ty1.Ty2 :: Kind1 -> Kind2
    /// ```
    ///
    /// otherwise
    /// ```text
    /// Gamma,TypeVar::Kind3 |-> Ty2 :: Kind2
    /// ---------------------------------------------
    /// Gamma |-> \TypeVar::Kind3.Ty2 :: Kind1 => Kind2
    /// ```
    #[must_use] pub fn kind_op_lam(bounded: bool) -> Self {
        let annot = if bounded {
            Symbol::less_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Type, 1))
        } else {
            Symbol::double_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Kind, 3))
        };
        let prem_env = Symbol::comma_sep(SpecialChar::Gamma, annot.clone());
        let prem_bound = vec![
            ConclusionRule::kinding(Symbol::sub(Symbol::Type, 1), Symbol::sub(Symbol::Kind, 1)),
            ConclusionRule::kinding(Symbol::sub(Symbol::Type, 2), Symbol::sub(Symbol::Kind, 2)),
        ];
        let prem_unbound = vec![
            ConclusionRule::kinding(Symbol::sub(Symbol::Type, 2), Symbol::sub(Symbol::Kind, 2))
                .with_env(prem_env),
        ];
        let conc_in = vec![
            SpecialChar::Lambda.into(),
            annot,
            SpecialChar::Dot.into(),
            Symbol::sub(Symbol::Type, 2),
        ];

        Self {
            premises: if bounded { prem_bound } else { prem_unbound },
            label: format!("K-OpLam{}", if bounded { "<:" } else { "" }),
            conclusion: ConclusionRule::kinding(
                conc_in,
                vec![
                    Symbol::sub(Symbol::Kind, 1),
                    SpecialChar::DoubleArrow.into(),
                    Symbol::sub(Symbol::Kind, 2),
                ],
            ),
        }
    }

    /// Derivation Rule for kinding Record Types
    /// ```text
    /// Gamma |-> Term :: *
    /// ------------------------------
    /// Gamma |-> { label : Term } :: *
    /// ```
    #[must_use] pub fn kind_rec() -> Self {
        Self {
            premises: vec![ConclusionRule::kinding(Symbol::Term, SpecialChar::Star)],
            label: "K-Rec".to_owned(),
            conclusion: ConclusionRule::kinding(
                Symbol::brack(Symbol::colon_sep(Symbol::Label, Symbol::Type)),
                SpecialChar::Star,
            ),
        }
    }

    /// Derivation Rule for kinding sum types
    /// ```text
    /// Gamma |-> Type1 :: *    Gamma |-> Type2 :: *
    /// -------------------------------------
    /// Gamma |-> Type1 + Type2 :: *
    /// ```
    #[must_use] pub fn kind_sum() -> Self {
        Self {
            premises: vec![
                ConclusionRule::kinding(Symbol::sub(Symbol::Type, 1), SpecialChar::Star),
                ConclusionRule::kinding(Symbol::sub(Symbol::Type, 2), SpecialChar::Star),
            ],
            label: "K-Sum".to_owned(),
            conclusion: ConclusionRule::kinding(
                vec![
                    Symbol::sub(Symbol::Type, 1),
                    SpecialChar::Plus.into(),
                    Symbol::sub(Symbol::Type, 2),
                ],
                SpecialChar::Star,
            ),
        }
    }

    /// Derivation Rule for kinding variables
    /// ```text
    /// Typevariable :: Kind in Gamma
    /// -----------------------------
    /// Gamma |-> TypeVariable :: Kind
    /// ```
    #[must_use] pub fn kind_var() -> Self {
        Self {
            premises: vec![ConclusionRule::lookup_env(Symbol::double_colon_sep(
                Symbol::Typevariable,
                Symbol::Kind,
            ))],
            label: "K-Var".to_owned(),
            conclusion: ConclusionRule::kinding(Symbol::Typevariable, Symbol::Kind),
        }
    }

    /// Derivation Rule for normalizing congruence
    /// the given functions constructs a symbol representing the outer type
    pub fn norm_cong<F>(conf_f: F) -> Self
    where
        F: Fn(Symbol) -> Symbol,
    {
        Self {
            premises: vec![ConclusionRule::eval(
                Symbol::sub(Symbol::Type, 1),
                Symbol::sub(Symbol::Type, 2),
            )],
            label: "Norm-Cong".to_owned(),
            conclusion: ConclusionRule::eval(
                conf_f(Symbol::sub(Symbol::Type, 1)),
                conf_f(Symbol::sub(Symbol::Type, 2)),
            ),
        }
    }

    ///Derivation Rule for operator application (beta reduction
    /// if bounded
    /// ```text
    /// Gamma |-> (\Typevar<:Type3.Type1) Type2  -> Type1[TypeVar -> Type2]
    /// ```
    ///
    /// otherwise
    /// ```text
    /// Gamma |-> (\Typevar::Kind.Type1) Type2 -> Type1[TypeVar -> Type2]
    /// ```
    #[must_use] pub fn norm_ap(bounded: bool) -> Self {
        let annot = if bounded {
            Symbol::less_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Type, 3))
        } else {
            Symbol::double_colon_sep(Symbol::Typevariable, Symbol::Kind)
        };
        Self {
            premises: vec![],
            label: format!("N-OpApp{}-Beta", if bounded { "<:" } else { "" }),
            conclusion: ConclusionRule::eval(
                vec![
                    SpecialChar::Lambda.into(),
                    annot,
                    SpecialChar::Dot.into(),
                    Symbol::sub(Symbol::Type, 1),
                    Symbol::sub(Symbol::Type, 2),
                ],
                vec![
                    Symbol::sub(Symbol::Type, 1),
                    Symbol::sqbrack(Symbol::mapto(
                        Symbol::Typevariable,
                        Symbol::sub(Symbol::Type, 2),
                    )),
                ],
            ),
        }
    }

    /// Derivation rule for congruence evaluation
    /// with a single premise
    /// The arguments to `cong_fun` are Term1 and Term2 so this should not be used in the
    /// constructed symbol
    /// That is `cong_fun = |sym| vec![Symbol::Value,SpecialChar::Space,sym]` will create
    /// ```text
    /// Gamma |-> value Term1 -> value Term2
    /// ```
    pub fn eval_cong<F, S>(cong_fun: F, lb: &str) -> Self
    where
        S: Into<Symbol>,
        F: Fn(Symbol) -> S,
    {
        Self {
            premises: vec![ConclusionRule::eval(
                Symbol::sub(Symbol::Term, 1),
                Symbol::sub(Symbol::Term, 2),
            )],
            label: lb.to_owned(),
            conclusion: ConclusionRule::eval(
                cong_fun(Symbol::sub(Symbol::Term, 1)).into(),
                cong_fun(Symbol::sub(Symbol::Term, 2)).into(),
            ),
        }
    }

    // Evaluation rule for evaluation steps
    // from -- label --> to
    pub fn eval<S1, S2>(from: S1, to: S2, lb: &str) -> Self
    where
        S1: Into<Symbol>,
        S2: Into<Symbol>,
    {
        Self {
            premises: vec![],
            label: lb.to_owned(),
            conclusion: ConclusionRule::eval(from, to),
        }
    }
}

impl fmt::Display for DerivationRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}",
            self.premises
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n"),
            self.label,
            self.conclusion
        )
    }
}
