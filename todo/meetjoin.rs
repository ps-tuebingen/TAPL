/*pub fn join(ty1: Type, ty2: Type) -> Type {
    match (ty1, ty2) {
        (Type::Top, _) => Type::Top,
        (_, Type::Top) => Type::Top,
        (Type::Bot, ty) => ty,
        (ty, Type::Bot) => ty,
        (
            Type::Fun {
                from: from1,
                to: to1,
            },
            Type::Fun {
                from: from2,
                to: to2,
            },
        ) => {
            let new_from = meet(*from1, *from2);
            let new_to = join(*to1, *to2);
            Type::Fun {
                from: Box::new(new_from),
                to: Box::new(new_to),
            }
        }
        (Type::Record(recs1), Type::Record(recs2)) => {
            let mut new_recs = HashMap::new();
            let mut new_labels: HashSet<&String> = recs1.keys().collect();
            new_labels = new_labels.union(&recs2.keys().collect()).cloned().collect();
            for label in new_labels.into_iter() {
                match (recs1.get(label), recs2.get(label)) {
                    (None, None) => continue,
                    (Some(ty), None) => {
                        new_recs.insert(label.clone(), ty.clone());
                    }
                    (None, Some(ty)) => {
                        new_recs.insert(label.clone(), ty.clone());
                    }
                    (Some(ty1), Some(ty2)) => {
                        let new_ty = join(ty1.clone(), ty2.clone());
                        new_recs.insert(label.clone(), new_ty);
                    }
                }
            }
            Type::Record(new_recs)
        }
        (Type::Variant(vars1), Type::Variant(vars2)) => {
            let mut new_labels: HashSet<&String> = vars1.iter().map(|(v, _)| v).collect();
            new_labels = new_labels
                .union(&vars2.iter().map(|(v, _)| v).collect())
                .cloned()
                .collect();
            let mut new_vars = vec![];
            for new_label in new_labels {
                match (
                    vars1.iter().find(|(v, _)| v == new_label),
                    vars2.iter().find(|(v, _)| v == new_label),
                ) {
                    (None, None) => continue,
                    (Some((_, ty)), None) => {
                        new_vars.push((new_label.clone(), ty.clone()));
                    }
                    (None, Some((_, ty))) => {
                        new_vars.push((new_label.clone(), ty.clone()));
                    }
                    (Some((_, ty1)), Some((_, ty2))) => {
                        let new_ty = join(ty1.clone(), ty2.clone());
                        new_vars.push((new_label.clone(), new_ty));
                    }
                }
            }
            Type::Variant(new_vars)
        }
        (Type::List(ty1), Type::List(ty2)) => {
            let new_ty = join(*ty1, *ty2);
            Type::List(Box::new(new_ty))
        }
        (Type::Ref(ty1), Type::Ref(ty2)) => {
            if is_subtype(&ty1, &ty2) && is_subtype(&ty2, &ty2) {
                Type::Ref(ty1)
            } else {
                Type::Top
            }
        }
        (Type::Ref(ty1), Type::Sink(ty2)) => {
            let new_ty = meet(*ty1, *ty2);
            Type::Sink(Box::new(new_ty))
        }
        (Type::Source(ty1), Type::Ref(ty2)) => {
            let new_ty = join(*ty1, *ty2);
            Type::Source(Box::new(new_ty))
        }
        (Type::Source(ty1), Type::Source(ty2)) => {
            let new_ty = join(*ty1, *ty2);
            Type::Source(Box::new(new_ty))
        }
        (Type::Sink(ty1), Type::Sink(ty2)) => {
            let new_ty = meet(*ty1, *ty2);
            Type::Sink(Box::new(new_ty))
        }
        (ty1, ty2) => {
            if is_subtype(&ty1, &ty2) {
                ty2
            } else if is_subtype(&ty2, &ty1) {
                ty1
            } else {
                Type::Top
            }
        }
    }
}*/

/*pub fn meet(ty1: Type, ty2: Type) -> Type {
    match (ty1, ty2) {
        (Type::Top, ty) => ty,
        (ty, Type::Top) => ty,
        (Type::Bot, _) => Type::Bot,
        (_, Type::Bot) => Type::Bot,
        (
            Type::Fun {
                from: from1,
                to: to1,
            },
            Type::Fun {
                from: from2,
                to: to2,
            },
        ) => {
            let new_from = join(*from1, *from2);
            let new_to = meet(*to1, *to2);
            Type::Fun {
                from: Box::new(new_from),
                to: Box::new(new_to),
            }
        }
        (Type::Record(recs1), Type::Record(recs2)) => {
            let mut new_labels: HashSet<&String> = recs1.keys().collect();
            new_labels = new_labels
                .intersection(&recs2.keys().collect())
                .cloned()
                .collect();
            let mut new_rec = HashMap::new();
            for label in new_labels.into_iter() {
                let ty1 = recs1.get(label).unwrap();
                let ty2 = recs2.get(label).unwrap();
                let new_ty = join(ty1.clone(), ty2.clone());
                new_rec.insert(label.clone(), new_ty);
            }
            Type::Record(new_rec)
        }
        (Type::Variant(vars1), Type::Variant(vars2)) => {
            let mut new_labels: HashSet<&String> = vars1.iter().map(|(v, _)| v).collect();
            new_labels = new_labels
                .intersection(&vars2.iter().map(|(v, _)| v).collect())
                .cloned()
                .collect();
            let mut new_vars = vec![];
            for new_label in new_labels.into_iter() {
                let (_, ty1) = vars1.iter().find(|(v, _)| v == new_label).unwrap();
                let (_, ty2) = vars2.iter().find(|(v, _)| v == new_label).unwrap();
                let new_ty = meet(ty1.clone(), ty2.clone());
                new_vars.push((new_label.clone(), new_ty));
            }
            Type::Variant(new_vars)
        }
        (Type::List(ty1), Type::List(ty2)) => {
            let new_ty = meet(*ty1, *ty2);
            Type::List(Box::new(new_ty))
        }
        (Type::Ref(ty1), Type::Ref(ty2)) => {
            if is_subtype(&ty1, &ty2) && is_subtype(&ty1, &ty2) {
                Type::Ref(ty1)
            } else {
                Type::Bot
            }
        }
        (Type::Source(ty1), Type::Ref(ty2)) => {
            let new_ty = meet(*ty1, *ty2);
            Type::Ref(Box::new(new_ty))
        }
        (Type::Ref(ty1), Type::Source(ty2)) => {
            let new_ty = meet(*ty1, *ty2);
            Type::Ref(Box::new(new_ty))
        }
        (Type::Sink(ty1), Type::Ref(ty2)) => {
            let new_ty = join(*ty1, *ty2);
            Type::Ref(Box::new(new_ty))
        }
        (Type::Ref(ty1), Type::Sink(ty2)) => {
            let new_ty = join(*ty1, *ty2);
            Type::Ref(Box::new(new_ty))
        }
        (Type::Source(ty1), Type::Source(ty2)) => {
            let new_ty = meet(*ty1, *ty2);
            Type::Source(Box::new(new_ty))
        }

        (Type::Sink(ty1), Type::Sink(ty2)) => {
            let new_ty = join(*ty1, *ty2);
            Type::Sink(Box::new(new_ty))
        }
        (ty1, ty2) => {
            if is_subtype(&ty1, &ty2) {
                ty1
            } else if is_subtype(&ty2, &ty1) {
                ty2
            } else {
                Type::Bot
            }
        }
    }
}*/
