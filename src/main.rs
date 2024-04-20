#[derive(PartialEq, Debug)]
enum Term {
    If(Box<Term>, Box<Term>, Box<Term>),
    True,
    False,
}

fn is_value(t: Term) -> bool {
    match t {
        Term::True => true,
        Term::False => true,
        Term::If(_t1, _t2, _t3) => false,
    }
}

fn e_if(t: Term) -> Term {
    match t {
        Term::If(t1, t2, t3) => match *t1 {
            Term::True => *t2,
            Term::False => *t3,
            _ => e_if(Term::If(Box::new(e_if(*t1)), t2, t3)),
        },
        // This is Error cuz e_if is not affected to term thats not IF
        _ => Term::True,
    }
}

fn main() {
    println!("Hello");
}

#[test]
fn test() {
    assert_eq!(is_value(Term::True), true);
    assert_eq!(is_value(Term::False), true);
    assert_eq!(
        e_if(Term::If(
            Box::new(Term::True),
            Box::new(Term::True),
            Box::new(Term::True)
        )),
        Term::True
    );
}
