use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    If(Box<Term>, Box<Term>, Box<Term>),
    True,
    False,
}

#[derive(PartialEq, Debug)]
pub struct CustomError {
    message: String,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.message)
    }
}

pub fn is_value(t: Term) -> Result<bool, CustomError> {
    match t {
        Term::True | Term::False => Ok(true),
        Term::If(_, _, _) => Ok(false),
        _ => Err(CustomError {
            message: "t is not good type".to_string(),
        }),
    }
}

pub fn e_iftrue(t: Term) -> Result<Term, CustomError> {
    match t {
        Term::If(t1, t2, _t3) => {
            if *t1 == Term::True {
                Ok(*t2)
            } else {
                Err(CustomError {
                    message: "t1 is not Term::True".to_string(),
                })
            }
        }
        _ => Err(CustomError {
            message: "t is not Term::If".to_string(),
        }),
    }
}

pub fn e_iffalse(t: Term) -> Result<Term, CustomError> {
    match t {
        Term::If(t1, _t2, t3) => {
            if *t1 == Term::False {
                Ok(*t3)
            } else {
                Err(CustomError {
                    message: "t1 is not Term::False".to_string(),
                })
            }
        }
        _ => Err(CustomError {
            message: "t is not Term::If".to_string(),
        }),
    }
}

fn e_if(t: Term) -> Result<Term, CustomError> {
    match t {
        Term::If(t1, t2, t3) => {
            if let Ok(evaluated_t1) = eval_1(*t1) {
                e_if(Term::If(Box::new(evaluated_t1), t2, t3))
            } else {
                Err(CustomError {
                    message: "t1 is not evaluated".to_string(),
                })
            }
        }
        _ => Err(CustomError {
            message: "t is not Term::If".to_string(),
        }),
    }
}

pub fn eval_1(t: Term) -> Result<Term, CustomError> {
    match t {
        Term::If(box Term::True, box _, box _) => e_iftrue(t),
        Term::If(box Term::False, box _, box _) => e_iffalse(t),
        Term::If(t1, t2, t3) => Ok(Term::If(Box::new(eval_1(*t1)?), t2, t3)),
        _ => Err(CustomError {
            message: "t is not evaluated".to_string(),
        }),
    }
}
