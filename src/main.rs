#![feature(box_patterns)]

use crate::untyped::*;

mod untyped;

fn main() {}

#[test]
fn untyped_test() {
    assert_eq!(is_value(Term::True), Ok(true));
    assert_eq!(is_value(Term::False), Ok(true));

    let s = Term::If(
        Box::new(Term::True),
        Box::new(Term::False),
        Box::new(Term::False),
    );
    let t = Term::If(Box::new(s), Box::new(Term::True), Box::new(Term::True));
    let u = Term::If(
        Box::new(Term::False),
        Box::new(Term::True),
        Box::new(Term::True),
    );

    assert_eq!(
        eval_1(Term::If(
            Box::new(t),
            Box::new(Term::False),
            Box::new(Term::False)
        )),
        Ok(Term::If(
            Box::new(u),
            Box::new(Term::False),
            Box::new(Term::False)
        ))
    );

    assert_eq!(
        eval_1(Term::Pred(Box::new(Term::Succ(Box::new(Term::Pred(
            Box::new(Term::Zero)
        )))))),
        Ok(Term::Pred(Box::new(Term::Succ(Box::new(Term::Zero)))))
    );

    assert_eq!(
        eval_1(Term::Succ(Box::new(Term::Pred(Box::new(Term::Zero))))),
        Ok((Term::Succ(Box::new(Term::Zero))))
    );

    assert_eq!(eval_1(Term::Pred(Box::new(Term::Zero))), Ok((Term::Zero)));
}
