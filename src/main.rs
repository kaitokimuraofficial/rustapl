#[derive(PartialEq, Debug)]
enum Term {
    TermTrue,
    TermFalse,
    Val(Val),
}

#[derive(PartialEq, Debug)]
enum Val {
    ValTrue,
    ValFalse,
}

fn is_val(t: Term) -> Val {
    match t {
        Term::TermTrue => Val::ValTrue,
        Term::TermFalse => Val::ValFalse,
        Term::Val(val) => val,
    }
}

fn main() {
    println!("Hello");
}

#[test]
fn test() {
    assert_eq!(is_val(Term::TermTrue), Val::ValTrue);
    assert_eq!(is_val(Term::TermFalse), Val::ValFalse);
}
