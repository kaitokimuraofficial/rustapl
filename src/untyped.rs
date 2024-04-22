use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    If(Box<Term>, Box<Term>, Box<Term>),
    True,
    False,
    Zero,
    Succ(Box<Term>),
    Pred(Box<Term>),
    IsZero(Box<Term>),
    Var(i8, i8),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
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
        Term::True | Term::False | Term::Abs(_, _) => Ok(true),
        Term::Var(_, _) => Ok(false),
        _ => is_numerable(t),
    }
}

pub fn is_numerable(t: Term) -> Result<bool, CustomError> {
    match t {
        Term::Zero => Ok(true),
        Term::Succ(box t1) => is_numerable(t1),
        Term::True | Term::False | Term::If(_, _, _) | Term::Pred(_) | Term::IsZero(_) => Ok(false),
        _ => Err(CustomError {
            message: "has undefined form.".to_string(),
        }),
    }
}

pub fn e_if_true(t: Term) -> Result<Term, CustomError> {
    match t {
        Term::If(box Term::True, t2, _t3) => Ok(*t2),
        _ => Err(CustomError {
            message: "does not have form of If(True, _, _).".to_string(),
        }),
    }
}

pub fn e_if_false(t: Term) -> Result<Term, CustomError> {
    match t {
        Term::If(box Term::False, _t2, t3) => Ok(*t3),
        _ => Err(CustomError {
            message: "does not have form of If(False, _, _).".to_string(),
        }),
    }
}

pub fn e_if(t: Term) -> Result<Term, CustomError> {
    match t {
        Term::If(t1, t2, t3) => {
            if let Ok(evaluated_t1) = eval_1(*t1) {
                Ok(Term::If(Box::new(evaluated_t1), t2, t3))
            } else {
                Err(CustomError {
                    message: "t1 can not be evaluated anymore.".to_string(),
                })
            }
        }
        _ => Err(CustomError {
            message: "does not have form of If(_, _, _).".to_string(),
        }),
    }
}

pub fn e_succ(t: Term) -> Result<Term, CustomError> {
    match t {
        Term::Succ(t1) => {
            if let Ok(evaluated_t1) = eval_1(*t1) {
                e_succ(evaluated_t1)
            } else {
                Err(CustomError {
                    message: "t1 can not be evaluated anymore.".to_string(),
                })
            }
        }
        _ => Err(CustomError {
            message: "does not have form of Succ(_).".to_string(),
        }),
    }
}

pub fn e_pred_zero(t: Term) -> Result<Term, CustomError> {
    match t {
        Term::Pred(box Term::Zero) => Ok(Term::Zero),
        _ => Err(CustomError {
            message: "does not have form of Pred(Zero).".to_string(),
        }),
    }
}

pub fn e_pred_succ(t: Term) -> Result<Term, CustomError> {
    match t {
        Term::Pred(box Term::Succ(box t1)) if is_numerable(t1.clone())? => Ok(t1),
        _ => Err(CustomError {
            message: "does not have form of Pred( Succ(Zero) ).".to_string(),
        }),
    }
}

pub fn e_pred(t: Term) -> Result<Term, CustomError> {
    match t {
        Term::Succ(t1) => {
            if let Ok(evaluated_t1) = eval_1(*t1) {
                Ok(Term::Pred(Box::new(evaluated_t1)))
            } else {
                Err(CustomError {
                    message: "t1 can not be evaluated anymore.".to_string(),
                })
            }
        }
        _ => Err(CustomError {
            message: "does not have form of Pred(_).".to_string(),
        }),
    }
}

pub fn e_iszero_zero(t: Term) -> Result<Term, CustomError> {
    match t {
        Term::IsZero(box Term::Zero) => Ok(Term::True),
        _ => Err(CustomError {
            message: "does not have form of IsZero(Zero).".to_string(),
        }),
    }
}

pub fn e_iszero_succ(t: Term) -> Result<Term, CustomError> {
    match t {
        Term::IsZero(box Term::Succ(_)) => Ok(Term::False),
        _ => Err(CustomError {
            message: "does not have form of IsZero( Succ(_) ).".to_string(),
        }),
    }
}

pub fn e_iszero(t: Term) -> Result<Term, CustomError> {
    match t {
        Term::IsZero(t1) => {
            if let Ok(evaluated_t1) = eval_1(*t1) {
                Ok(Term::IsZero(Box::new(evaluated_t1)))
            } else {
                Err(CustomError {
                    message: "t1 can not be evaluated anymore.".to_string(),
                })
            }
        }
        _ => Err(CustomError {
            message: "does not have form of IsZero(_).".to_string(),
        }),
    }
}

fn internal_shift(cutoff: i8, d: i8, t: Term) -> Result<Term, CustomError> {
    match t {
        Term::Var(idx, len) => {
            if idx >= cutoff {
                Ok(Term::Var(idx + d, len + d))
            } else {
                Ok(Term::Var(idx, len + d))
            }
        }
        Term::Abs(name, box t1) => Ok(Term::Abs(
            name,
            Box::new(internal_shift(cutoff + 1, d, t1)?),
        )),
        Term::App(box t1, box t2) => Ok(Term::App(
            Box::new(internal_shift(cutoff, d, t1)?),
            Box::new(internal_shift(cutoff, d, t2)?),
        )),
        _ => Err(CustomError {
            message: "can not be internally shifted anymore.".to_string(),
        }),
    }
}

pub fn term_shift(d: i8, t: Term) -> Result<Term, CustomError> {
    internal_shift(0, d, t)
}

fn internal_subst(cutoff: i8, j: i8, s: Term, t: Term) -> Result<Term, CustomError> {
    match t {
        Term::Var(idx, len) => {
            if idx == cutoff + j {
                Ok(term_shift(cutoff, s)?)
            } else {
                Ok(Term::Var(idx, len))
            }
        }
        Term::Abs(name, box t1) => Ok(Term::Abs(
            name,
            Box::new(internal_subst(cutoff + 1, j, s, t1)?),
        )),
        Term::App(box t1, box t2) => Ok(Term::App(
            Box::new(internal_subst(cutoff, j, s.clone(), t1)?),
            Box::new(internal_subst(cutoff, j, s.clone(), t2)?),
        )),
        _ => Err(CustomError {
            message: "can not be internally substituted anymore.".to_string(),
        }),
    }
}

// term_subst j s t := [j↦s]t
pub fn term_subst(j: i8, s: Term, t: Term) -> Result<Term, CustomError> {
    internal_subst(0, j, s, t)
}

pub fn eval_1(t: Term) -> Result<Term, CustomError> {
    match t {
        Term::If(box Term::True, _, _) => e_if_true(t),
        Term::If(box Term::False, _, _) => e_if_false(t),
        Term::If(box t1, t2, t3) => Ok(Term::If(Box::new(eval_1(t1)?), t2, t3)),
        Term::Succ(box t1) => Ok(Term::Succ(Box::new(eval_1(t1)?))),
        Term::Pred(box Term::Zero) => e_pred_zero(t),
        Term::Pred(box Term::Succ(box t1)) if is_numerable(t1.clone())? => Ok(t1),
        Term::Pred(box t1) => Ok(Term::Pred(Box::new(eval_1(t1)?))),
        Term::IsZero(box Term::Zero) => e_iszero_zero(t),
        Term::IsZero(box Term::Succ(box t1)) => e_iszero_succ(t1),
        Term::IsZero(box t1) => Ok(Term::IsZero(Box::new(eval_1(t1)?))),
        // TODO: Add some kinds of Term::App evaluation.
        _ => Err(CustomError {
            message: "t is not evaluated".to_string(),
        }),
    }
}
