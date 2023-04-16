pub use term::Subterm;
enum Term {
    Variable { de_bruijn_index: usize },
    Lambda { binder_type: Subterm, body: Subterm },
    PiType { binder_type: Subterm, body: Subterm },
}

mod term {
    use super::Term;
    use std::rc::Rc;

    #[derive(Clone)]
    pub struct Subterm {
        ptr: Rc<Term>,
    }

    impl From<Term> for Subterm {
        fn from(term: Term) -> Self {
            Self { ptr: Rc::new(term) }
        }
    }

    impl AsRef<Term> for Subterm {
        fn as_ref(&self) -> &Term {
            self.ptr.as_ref()
        }
    }

    impl PartialEq for Subterm {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.ptr, &other.ptr)
        }
    }

    impl Eq for Subterm {}
}

impl Subterm {
    pub fn new_variable(de_bruijn_index: usize) -> Self {
        Term::Variable { de_bruijn_index }.into()
    }

    pub fn new_lambda(binder_type: Subterm, body: Subterm) -> Self {
        Term::Lambda { binder_type, body }.into()
    }

    pub fn new_pi_type(binder_type: Subterm, body: Subterm) -> Self {
        Term::PiType { binder_type, body }.into()
    }

    fn replace_variable(&self, new_de_bruijn_index: usize) -> Self {
        if let Term::Variable { de_bruijn_index } = self.as_ref() {
            if de_bruijn_index == &new_de_bruijn_index {
                return self.clone();
            }
        }
        Self::new_variable(new_de_bruijn_index)
    }

    fn replace_lambda(&self, new_binder_type: Subterm, new_body: Subterm) -> Self {
        if let Term::Lambda { binder_type, body } = self.as_ref() {
            if binder_type == &new_binder_type && body == &new_body {
                return self.clone();
            }
        }
        Self::new_lambda(new_binder_type, new_body)
    }

    fn replace_pi_type(&self, new_binder_type: Subterm, new_body: Subterm) -> Self {
        if let Term::Lambda { binder_type, body } = self.as_ref() {
            if binder_type == &new_binder_type && body == &new_body {
                return self.clone();
            }
        }
        Self::new_pi_type(new_binder_type, new_body)
    }

    /// `level` musn't be 0
    pub fn substitute(&self, level: usize, replacement: &Self) -> Self {
        match self.as_ref() {
            &Term::Variable { de_bruijn_index } => {
                if de_bruijn_index == level {
                    replacement.increase_de_bruijn_index(level - 1)
                } else if de_bruijn_index > level {
                    Term::Variable {
                        de_bruijn_index: de_bruijn_index - 1,
                    }
                    .into()
                } else {
                    self.clone()
                }
            }
            Term::Lambda { binder_type, body } => self.replace_lambda(
                binder_type.substitute(level, replacement),
                body.substitute(level + 1, replacement),
            ),
            Term::PiType { binder_type, body } => self.replace_pi_type(
                binder_type.substitute(level, replacement),
                body.substitute(level + 1, replacement),
            ),
        }
    }

    fn increase_de_bruijn_index(&self, increment: usize) -> Self {
        match self.as_ref() {
            Term::Variable { de_bruijn_index } => {
                self.replace_variable(de_bruijn_index + increment)
            }
            Term::Lambda { binder_type, body } => self.replace_lambda(
                binder_type.increase_de_bruijn_index(increment),
                body.increase_de_bruijn_index(increment),
            ),
            Term::PiType { binder_type, body } => self.replace_pi_type(
                binder_type.increase_de_bruijn_index(increment),
                body.increase_de_bruijn_index(increment),
            ),
        }
    }

    pub fn ty(&self) -> &Self {
        unimplemented!()
    }
}
