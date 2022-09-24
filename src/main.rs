fn main() {
    use regex::Regex;
    r"(\p{L}[\p{L}\p{Nd}\p{No}]*)\s*";
    r"(:=|:|∀|λ|,|\(|\))\s*";
    r"\s*";
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    assert!(re.is_match("2014-01-01"));
}

mod Deep {
    use std::rc::Rc;

    #[derive(Clone)]
    pub enum Term {
        Sort(usize),
        /// Consider De Bruijn index to start form 0?
        Binder(usize),
        λ(Rc<Term>, Rc<Term>),
        Π(Rc<Term>, Rc<Term>),
    }

    /// `term` must already be type-checked before calling this function
    fn substitute<'a>(this: Rc<Term>, index: usize, term: &'a Rc<Term>) -> Rc<Term> {
        match this.as_ref() {
            Term::Sort(_) => this,
            &Term::Binder(de_bruijn_index) => {
                if de_bruijn_index == index {
                    Rc::clone(term)
                } else if de_bruijn_index > index {
                    Rc::new(Term::Binder(de_bruijn_index - 1))
                } else {
                    this
                }
            }
            Term::λ(binder_type, body) => {
                let new_binder_type = substitute(Rc::clone(binder_type), index, term);
                let new_body = substitute(Rc::clone(body), index + 1, term);
                if Rc::ptr_eq(&binder_type, &new_binder_type) && Rc::ptr_eq(&body, &new_body) {
                    this
                } else {
                    Rc::new(Term::λ(new_binder_type, new_body))
                }
            }
            Term::Π(binder_type, body) => {
                let new_binder_type = substitute(Rc::clone(binder_type), index, term);
                let new_body = substitute(Rc::clone(body), index + 1, term);
                if Rc::ptr_eq(&binder_type, &new_binder_type) && Rc::ptr_eq(&body, &new_body) {
                    this
                } else {
                    Rc::new(Term::Π(new_binder_type, new_body))
                }
            }
        }
    }

    /// `offset` must make the type of the return value valid
    fn levelUp(this: Rc<Term>, offset: usize) -> Rc<Term> {
        match this.as_ref() {
            Term::Sort(_) => this,
            &Term::Binder(de_bruijn_index) => {
                if offset == 0 {
                    this
                } else {
                    Rc::new(Term::Binder(de_bruijn_index + offset))
                }
            }
            Term::λ(binder_type, body) => {
                let new_binder_type = levelUp(Rc::clone(binder_type), offset);
                let new_body = levelUp(Rc::clone(body), offset);
                if Rc::ptr_eq(&binder_type, &new_binder_type) && Rc::ptr_eq(&body, &new_body) {
                    this
                } else {
                    Rc::new(Term::λ(new_binder_type, new_body))
                }
            }
            Term::Π(binder_type, body) => {
                let new_binder_type = levelUp(Rc::clone(binder_type), offset);
                let new_body = levelUp(Rc::clone(body), offset);
                if Rc::ptr_eq(&binder_type, &new_binder_type) && Rc::ptr_eq(&body, &new_body) {
                    this
                } else {
                    Rc::new(Term::Π(new_binder_type, new_body))
                }
            }
        }
    }

    /// type-check argument in this function
    pub fn try_apply(function: &Term, argument: &Term) -> Result<Rc<Term>, ()> {
        if let Term::λ(_, body) = function {
            Ok(substitute(Rc::clone(body), 1, &Rc::new(argument.clone())))
        } else {
            Err(())
        }
    }
}

mod Surface {
    pub enum Term {
        Var(String),
        App(Box<Term>, Box<Term>),
        λ(Box<Term>, Box<Term>),
        Π(Box<Term>, Box<Term>),
    }
}
