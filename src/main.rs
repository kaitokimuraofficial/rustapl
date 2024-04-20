#![feature(box_patterns)]

use crate::untyped::Term;

mod untyped;

fn main() {
    let pp = untyped::Term::Pred(Box::new(untyped::Term::Succ(Box::new(
        untyped::Term::Pred(Box::new(untyped::Term::Zero)),
    ))));
    println!(
        "Pred ( Succ ( Pred ( Zero ) ) ) is {:?}",
        untyped::eval_1(pp)
    );

    let pp2 = untyped::Term::Succ(Box::new(untyped::Term::Pred(Box::new(untyped::Term::Zero))));
    println!(
        "Succ ( Pred ( Zero ) ) is {:?}",
        untyped::eval_1(pp2).unwrap()
    );

    let pp3 = untyped::Term::Pred(Box::new(untyped::Term::Zero));
    println!("Pred ( Zero ) is {:?}", untyped::eval_1(pp3).unwrap());
}

#[test]
fn untyped_test() {
    assert_eq!(untyped::is_value(untyped::Term::True), Ok(true));
    assert_eq!(untyped::is_value(untyped::Term::False), Ok(true));

    let s = untyped::Term::If(
        Box::new(untyped::Term::True),
        Box::new(untyped::Term::False),
        Box::new(untyped::Term::False),
    );
    let t = untyped::Term::If(
        Box::new(s),
        Box::new(untyped::Term::True),
        Box::new(untyped::Term::True),
    );
    let u = untyped::Term::If(
        Box::new(untyped::Term::False),
        Box::new(untyped::Term::True),
        Box::new(untyped::Term::True),
    );

    assert_eq!(
        untyped::eval_1(untyped::Term::If(
            Box::new(t),
            Box::new(untyped::Term::False),
            Box::new(untyped::Term::False)
        )),
        Ok(untyped::Term::If(
            Box::new(u),
            Box::new(untyped::Term::False),
            Box::new(untyped::Term::False)
        ))
    );

    let pp = untyped::Term::Pred(Box::new(untyped::Term::Succ(Box::new(
        untyped::Term::Pred(Box::new(untyped::Term::Zero)),
    ))));
    let pp2 = untyped::Term::Pred(Box::new(untyped::Term::Succ(Box::new(untyped::Term::Zero))));
    assert_eq!(untyped::eval_1(pp), Ok(pp2));
}
