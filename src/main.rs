#![feature(box_patterns)]

mod untyped;

fn main() {
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


}
