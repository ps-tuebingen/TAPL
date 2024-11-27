use untyped_lambda::{
    eval::eval,
    examples::{
        id,
        num::{c1, cn, plus, succ},
        tree::{fold, leaf, node},
    },
    parse::parse,
    terms::Term,
};
fn main() {
    let sum_leaves = Term::App(
        Box::new(Term::App(
            Box::new(fold()),
            Box::new(Term::Lambda("v".to_owned(), Box::new(plus()))),
        )),
        Box::new(id()),
    );
    let map = parse(&mut "\\h.\\t.\\f.\\g.(t (\\v. f (h v))) h".to_owned()).unwrap();
    let size = Term::App(
        Box::new(Term::App(
            Box::new(fold()),
            Box::new(Term::Lambda(
                "v".to_owned(),
                Box::new(Term::Lambda(
                    "l".to_owned(),
                    Box::new(Term::Lambda(
                        "r".to_owned(),
                        Box::new(Term::App(
                            Box::new(Term::App(Box::new(plus()), Box::new(c1()))),
                            Box::new(Term::App(
                                Box::new(Term::App(
                                    Box::new(plus()),
                                    Box::new(Term::Var("l".to_owned())),
                                )),
                                Box::new(Term::Var("r".to_owned())),
                            )),
                        )),
                    )),
                )),
            )),
        )),
        Box::new(Term::Lambda("x".to_owned(), Box::new(c1()))),
    );

    let example_tree = parse(&mut format!(
        "(({} ({})) ((({} ({})) ({} {})) ({} {}))) ({} {})",
        node(),
        cn(3),
        node(),
        cn(2),
        leaf(),
        cn(4),
        leaf(),
        cn(1),
        leaf(),
        cn(5)
    ))
    .unwrap();

    println!(
        "sum leaves: {}",
        eval(Term::App(
            Box::new(sum_leaves),
            Box::new(example_tree.clone())
        ))
    );
    println!(
        "map: {}",
        eval(Term::App(
            Box::new(Term::App(Box::new(map), Box::new(succ()))),
            Box::new(example_tree.clone())
        ))
    );
    println!(
        "size: {}",
        eval(Term::App(Box::new(size), Box::new(example_tree)))
    );
}
