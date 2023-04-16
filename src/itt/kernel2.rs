pub enum Term {
    Variable(Variable),
    Universe(Universe),
}

type Type = Term;

pub struct Variable(String);
impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub struct Universe;

pub struct Binder {
    pub variable: Variable,
    pub r#type: Type,
}

mod ctx {
    use std::ops::Deref;

    use super::{Binder, Type, Variable};

    pub struct Assumption<'a> {
        pub binder: Binder,
        context: &'a Context<'a>,
    }
    impl<'a> Deref for Assumption<'a> {
        type Target = Binder;
        fn deref(&self) -> &Self::Target {
            &self.binder
        }
    }

    pub enum Context<'a> {
        Empty,
        More(Assumption<'a>),
    }
    impl<'a> IntoIterator for &'a Context<'a> {
        type Item = &'a Assumption<'a>;
        type IntoIter = ContextIter<'a>;
        fn into_iter(self) -> Self::IntoIter {
            ContextIter(self)
        }
    }
    impl<'a> From<Assumption<'a>> for Context<'a> {
        fn from(assumption: Assumption<'a>) -> Self {
            Context::More(assumption)
        }
    }
    impl<'a> Context<'a> {
        pub fn emp() -> Self {
            Context::Empty
        }
        pub fn ext(&'a self, variable: Variable, r#type: Type) -> Result<Self, ()> {
            if self.contains(&variable) {
                Ok(Assumption {
                    binder: Binder { variable, r#type },
                    context: self,
                }
                .into())
            } else {
                Err(())
            }
        }
        fn contains(&self, variable: &Variable) -> bool {
            self.into_iter()
                .any(|assumption| &assumption.variable == variable)
        }
    }

    pub struct ContextIter<'a>(&'a Context<'a>);
    impl<'a> Iterator for ContextIter<'a> {
        type Item = &'a Assumption<'a>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.0 {
                Context::Empty => None,
                Context::More(assumption) => {
                    self.0 = assumption.context; // Increment iterator
                    Some(assumption)
                }
            }
        }
    }
}
use ctx::Context;

enum Judgement<'a> {
    /// If the variants of `Context` are made private, `Context` is guaranteed
    /// to be well formed as its constructors are `Context::emp` and `Context::ext`.
    WellFormed(Context<'a>),
    Inhabitance(&'a Context<'a>, Binder),
    Equality(&'a Context<'a>, Equality),
}

/// Judgemental equality
struct Equality(Term, Term, Type);
