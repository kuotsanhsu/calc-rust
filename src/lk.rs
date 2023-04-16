// mod term;
// mod term2;
// pub use term2::kernel::{Sequent, Term};

pub mod kernel {
    pub enum Term<'a> {
        Var(String),
        Not(&'a Term<'a>),
        And(&'a Term<'a>, &'a Term<'a>),
        Or(&'a Term<'a>, &'a Term<'a>),
        Then(&'a Term<'a>, &'a Term<'a>),
    }

    #[derive(Clone)]
    pub struct Sequent<'a> {
        left: Vec<&'a Term<'a>>,
        right: Vec<&'a Term<'a>>,
    }

    impl<'a> Sequent<'a> {
        pub fn new(left: Vec<&'a Term>, right: Vec<&'a Term>) -> Self {
            Self { left, right }
        }

        pub fn left_iter(&self) -> std::slice::Iter<&Term> {
            self.left.iter()
        }

        pub fn right_iter(&self) -> std::slice::Iter<&Term> {
            self.right.iter()
        }
    }

    //     pub fn focus_left(
    //         &self,
    //         index: usize,
    //     ) -> LeftRule<
    //         impl Fn() -> Sequent + '_,
    //         impl Fn() -> Sequent + '_,
    //         impl Fn() -> (Sequent, Sequent) + '_,
    //         impl Fn() -> Sequent + '_,
    //         impl Fn() -> Sequent + '_,
    //     > {
    //         if let Some(focused_term) = self.left.get(index) {
    //             let weakening = move || {
    //                 let mut new = self.clone();
    //                 new.left.remove(index);
    //                 new
    //             };
    //             match focused_term.deref() {
    //                 Term::Var(_) => LeftRule::Var { weakening },
    //                 Term::Not(term) => LeftRule::Not {
    //                     weakening,
    //                     intro: move || {
    //                         let mut new = weakening();
    //                         new.right.push(term.clone());
    //                         new
    //                     },
    //                 },
    //                 Term::Or(left, right) => LeftRule::Or {
    //                     weakening,
    //                     intro: move || {
    //                         (
    //                             {
    //                                 let mut new = self.clone();
    //                                 new.left[index] = left.clone();
    //                                 new
    //                             },
    //                             {
    //                                 let mut new = self.clone();
    //                                 new.left[index] = right.clone();
    //                                 new
    //                             },
    //                         )
    //                     },
    //                 },
    //                 Term::And(left, right) => LeftRule::And {
    //                     weakening,
    //                     inl: move || {
    //                         let mut new = self.clone();
    //                         new.left[index] = left.clone();
    //                         new
    //                     },
    //                     inr: move || {
    //                         let mut new = self.clone();
    //                         new.left[index] = right.clone();
    //                         new
    //                     },
    //                 },
    //                 Term::Then(_, _) => LeftRule::Then { weakening },
    //             }
    //         } else {
    //             LeftRule::None
    //         }
    //     }

    //     pub fn focus_right(
    //         &self,
    //         index: usize,
    //     ) -> RightRule<
    //         impl Fn() -> Sequent + '_,
    //         impl Fn() -> Sequent + '_,
    //         impl Fn() -> (Sequent, Sequent) + '_,
    //         impl Fn() -> Sequent + '_,
    //         impl Fn() -> Sequent + '_,
    //         impl Fn() -> Sequent + '_,
    //     > {
    //         if let Some(focused_term) = self.right.get(index) {
    //             let weakening = move || {
    //                 let mut new = self.clone();
    //                 new.right.remove(index);
    //                 new
    //             };
    //             match focused_term.deref() {
    //                 Term::Var(_) => RightRule::Var { weakening },
    //                 Term::Not(term) => RightRule::Not {
    //                     weakening,
    //                     intro: move || {
    //                         let mut new = weakening();
    //                         new.left.push(term.clone());
    //                         new
    //                     },
    //                 },
    //                 Term::And(left, right) => RightRule::And {
    //                     weakening,
    //                     intro: move || {
    //                         (
    //                             {
    //                                 let mut new = self.clone();
    //                                 new.right[index] = left.clone();
    //                                 new
    //                             },
    //                             {
    //                                 let mut new = self.clone();
    //                                 new.right[index] = right.clone();
    //                                 new
    //                             },
    //                         )
    //                     },
    //                 },
    //                 Term::Or(left, right) => RightRule::Or {
    //                     weakening,
    //                     inl: move || {
    //                         let mut new = self.clone();
    //                         new.right[index] = left.clone();
    //                         new
    //                     },
    //                     inr: move || {
    //                         let mut new = self.clone();
    //                         new.right[index] = right.clone();
    //                         new
    //                     },
    //                 },
    //                 Term::Then(left, right) => RightRule::Then {
    //                     weakening,
    //                     intro: move || {
    //                         let mut new = self.clone();
    //                         new.left.push(left.clone());
    //                         new.right[index] = right.clone();
    //                         new
    //                     },
    //                 },
    //             }
    //         } else {
    //             RightRule::None
    //         }
    //     }
    // }

    // enum LeftRule<
    //     Weakening: Fn() -> Sequent,
    //     IntroInv: Fn() -> Sequent,
    //     IntroPair: Fn() -> (Sequent, Sequent),
    //     IntroLeft: Fn() -> Sequent,
    //     IntroRight: Fn() -> Sequent,
    // > {
    //     None,
    //     Var {
    //         weakening: Weakening,
    //     },
    //     Not {
    //         weakening: Weakening,
    //         intro: IntroInv,
    //     },
    //     Or {
    //         weakening: Weakening,
    //         intro: IntroPair,
    //     },
    //     And {
    //         weakening: Weakening,
    //         inl: IntroLeft,
    //         inr: IntroRight,
    //     },
    //     Then {
    //         weakening: Weakening,
    //     },
    // }

    // enum RightRule<
    //     Weakening: Fn() -> Sequent,
    //     IntroInv: Fn() -> Sequent,
    //     IntroPair: Fn() -> (Sequent, Sequent),
    //     IntroLeft: Fn() -> Sequent,
    //     IntroRight: Fn() -> Sequent,
    //     IntroCut: Fn() -> Sequent,
    // > {
    //     None,
    //     Var {
    //         weakening: Weakening,
    //     },
    //     Not {
    //         weakening: Weakening,
    //         intro: IntroInv,
    //     },
    //     And {
    //         weakening: Weakening,
    //         intro: IntroPair,
    //     },
    //     Or {
    //         weakening: Weakening,
    //         inl: IntroLeft,
    //         inr: IntroRight,
    //     },
    //     Then {
    //         weakening: Weakening,
    //         intro: IntroCut,
    //     },
    // }
}

use std::fmt;
use std::fmt::Display;

use kernel::Term;
impl Term<'_> {
    fn precedence(&self) -> i8 {
        match self {
            Term::Var(_) => 0,
            Term::Not(_) => 1,
            Term::And(_, _) => 2,
            Term::Or(_, _) => 3,
            Term::Then(_, _) => 4,
        }
    }
    fn display(&self, f: &mut fmt::Formatter, min_precedence: i8) -> fmt::Result {
        let precedence = self.precedence();
        let to_parenthesize = precedence > min_precedence;
        if to_parenthesize {
            write!(f, "(")?;
        }
        match self {
            Term::Var(name) => write!(f, "{}", name),
            Term::Not(term) => {
                write!(f, "¬")?;
                term.display(f, precedence)
            }
            Term::And(left, right) => {
                left.display(f, precedence)?;
                write!(f, " ∧ ")?;
                right.display(f, precedence - 1)
            }
            Term::Or(left, right) => {
                left.display(f, precedence)?;
                write!(f, " ∨ ")?;
                right.display(f, precedence - 1)
            }
            Term::Then(left, right) => {
                left.display(f, precedence - 1)?;
                write!(f, " → ")?;
                right.display(f, precedence)
            }
        }?;
        if to_parenthesize {
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl fmt::Display for Term<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.display(f, i8::MAX)
    }
}

use kernel::Sequent;

fn display_terms(mut terms: std::slice::Iter<&Term>, f: &mut fmt::Formatter) -> fmt::Result {
    if let Some(&term) = terms.next() {
        term.fmt(f)?;
        for &term in terms {
            write!(f, ", {}", term)?;
        }
    } else {
        write!(f, "•")?;
    }
    Ok(())
}

impl fmt::Display for Sequent<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        display_terms(self.left_iter(), f)?;
        write!(f, " ⊢ ")?;
        display_terms(self.right_iter(), f)
    }
}
