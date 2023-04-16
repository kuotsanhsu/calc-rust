mod normal_form;

pub use term::Subterm;
enum Term {
    Sort(usize),
    Variable(String),
    Application(Subterm, Subterm),
    Lambda {
        binder_name: String,
        binder_type: Subterm,
        body: Subterm,
    },
    PiType {
        binder_name: String,
        binder_type: Subterm,
        body: Subterm,
    },
}

mod term {
    use super::normal_form::Subterm as NormalForm;
    use super::Term;

    #[derive(Clone)]
    pub struct Subterm {
        ptr: std::rc::Rc<Term>,
        normal_form: NormalForm,
    }

    impl From<Term> for Subterm {
        fn from(term: Term) -> Self {
            let normal_form = match &term {
                Term::Sort(i) => NormalForm::new_variable(*i), // FIXME: this is wrong
                // α-conversion
                Term::Variable(_) => NormalForm::new_variable(0),
                // β-reduction
                Term::Application(left, right) => {
                    // Application. Should typecheck.
                    left.normal_form.substitute(0, &right.normal_form)
                }
                Term::Lambda {
                    binder_name,
                    binder_type,
                    body,
                } => NormalForm::new_lambda(
                    binder_type.normal_form.clone(),
                    body.normal_form.clone(),
                ),
                Term::PiType {
                    binder_name,
                    binder_type,
                    body,
                } => NormalForm::new_pi_type(
                    binder_type.normal_form.clone(),
                    body.normal_form.clone(),
                ),
            };

            Self {
                normal_form,
                ptr: std::rc::Rc::new(term),
            }
        }
    }

    impl AsRef<Term> for Subterm {
        fn as_ref(&self) -> &Term {
            self.ptr.as_ref()
        }
    }

    impl PartialEq for Subterm {
        fn eq(&self, other: &Self) -> bool {
            self.normal_form == other.normal_form
        }
    }

    impl Eq for Subterm {}
}

impl Subterm {
    pub fn new_sort(universe: usize) -> Self {
        Term::Sort(universe).into()
    }

    pub fn new_variable(name: String) -> Self {
        Term::Variable(name).into()
    }

    pub fn new_application(left: Subterm, right: Subterm) -> Self {
        Term::Application(left, right).into()
    }

    pub fn new_lambda(binder_name: String, binder_type: Subterm, body: Subterm) -> Self {
        Term::Lambda {
            binder_name,
            binder_type,
            body,
        }
        .into()
    }

    pub fn new_pi_type(binder_name: String, binder_type: Subterm, body: Subterm) -> Self {
        Term::PiType {
            binder_name,
            binder_type,
            body,
        }
        .into()
    }
}

struct ContextNode {
    variable: String,
    ty: Subterm,
    outer: Context,
}
#[derive(Clone)]
enum Context {
    Nil,
    Cons(std::rc::Rc<ContextNode>),
}

impl Context {
    fn free_of(&self, variable: &str) -> bool {
        self.position(variable).is_none()
    }

    fn assume(&self, variable: &str, ty: Subterm) -> Self {
        Context::Cons(std::rc::Rc::new(ContextNode {
            variable: variable.into(),
            ty,
            outer: self.clone(),
        }))
    }

    fn position(&self, variable: &str) -> Option<usize> {
        let mut position: usize = 0;
        let mut context = self;
        while let Context::Cons(context_node) = context {
            if context_node.variable == variable {
                return Some(position);
            }
            position += 1;
            context = &context_node.outer;
        }
        return None;
    }

    fn find(&self, variable: &str) -> Option<&Subterm> {
        let mut context = self;
        while let Context::Cons(context_node) = context {
            if context_node.variable == variable {
                return Some(&context_node.ty);
            }
            context = &context_node.outer;
        }
        return None;
    }

    pub fn axiom(&self, universe: usize) -> Consequent {
        Consequent(Subterm::new_sort(universe), Subterm::new_sort(universe + 1))
    }

    pub fn start(&self, x: &str) -> Result<Consequent, ()> {
        if let Some(ty) = self.find(x) {
            Ok(Consequent(Subterm::new_variable(x.into()), ty.clone()))
        } else {
            Err(())
        }
    }

    pub fn weakening(&self, x: &str, A: &Subterm, s: &Subterm) -> Result<Consequent, ()> {
        unimplemented!()
    }
}

pub struct Consequent(Subterm, Subterm);

// pub struct Judgement {
//     context: Context,
//     term: Subterm,
//     ty: Subterm,
// }

// impl Judgement {
//     fn start(a_type: &Self, variable: String) -> Result<Self, ()> {
//         if let Self {
//             context,
//             term: ty,
//             ty: Term::Sort(ref _),
//         } = a_type
//         {
//             if context.free_of(&variable) {
//                 return Ok(Self {
//                     context: context.assume(variable.clone(), ty.clone()),
//                     term: Subterm::new_variable(variable),
//                     ty: ty.clone(),
//                 });
//             }
//         }
//         return Err(());
//     }

//     fn weakening(a_term: &Self, a_type: &Self, variable: String) -> Result<Self, ()> {
//         let Self(context, ty, sort) = a_type;
//         // TODO: also check that the contexts of `a_term` and `a_type` agree?
//         if let Term::Sort(_) = sort.as_ref() {
//             if context.free_of(&variable) {
//                 return Ok(Self(
//                     context.assume(variable.clone(), ty.clone()),
//                     a_term.1.clone(),
//                     a_term.2.clone(),
//                 ));
//             }
//         }
//         return Err(());
//     }

//     fn product(a_type: &Self, b_type: &Self, variable: String) -> Result<Self, ()> {
//         let Self(Γ, A, s1) = a_type;
//         let Self(Δ, B, s2) = b_type;
//         let x = variable;
//         // TODO: also check that `Γ` and `Δ - {x:A}` agree?
//         if s1.is_sort() && s2.is_sort() && !Δ.free_of(&x) {
//             Ok(Self(
//                 Γ.clone(),
//                 Subterm::new_pi_type(x.clone(), A.clone(), B.clone()),
//                 s2.clone(),
//             ))
//         } else {
//             Err(())
//         }
//     }

//     fn application(fun: &Self, arg: &Self) -> Result<Self, ()> {
//         let Self(Γ, M, P) = fun;
//         let Self(Δ, N, A) = arg;
//         if let Term::PiType {
//             binder_name: x,
//             binder_type,
//             body: B,
//         } = P.as_ref()
//         {
//             if binder_type == A {
//                 return Ok(Self(
//                     Γ.clone(),
//                     Subterm::new_application(M.clone(), N.clone()),
//                     B.clone(),
//                 ));
//             }
//         }
//         return Err(());
//     }

//     fn conversion(a_term: &Self, a_type: &Self) -> Result<Self, ()> {
//         let Self(Γ, M, A) = a_term;
//         let Self(Δ, B, s) = a_type;
//         if s.is_sort() && A == B {
//             Ok(Self(Γ.clone(), M.clone(), B.clone()))
//         } else {
//             Err(())
//         }
//     }
// }
