use std::io;
use std::io::Write;
use std::vec;

pub mod itt;

fn main() {
    // use regex::Regex;
    // r"(\p{L}[\p{L}\p{Nd}\p{No}]*)\s*";
    // r"(:=|:|∀|λ|,|\(|\))\s*";
    // r"\s*";
    // let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    // assert!(re.is_match("2014-01-01"));

    // use coc::kernel::Subterm as Term;
    // let term1 = Term::new_application(
    //     Term::new_lambda(
    //         "x".into(),
    //         Term::new_variable("y".into()),
    //         Term::new_variable("x".into()),
    //     ),
    //     Term::new_variable("y".into()),
    // );
    // let term2 = Term::new_variable("z".into());

    use crate::kernel::{LeftRule, RightRule};

    let a = &Term::Var("A".into());
    let b = &Term::Var("B".into());
    let c = &Term::Var("C".into());

    let a_and_b = &Term::And(a, b);
    let b_and_a = &Term::And(b, a);
    let a_and_b_and_c = &Term::And(a_and_b, c);
    let abc_then_ba = &Term::Then(a_and_b_and_c, b_and_a);

    let mut goals = vec![Sequent::new(vec![], vec![abc_then_ba])];
    while let Some(goal) = goals.last_mut() {
        println!("{}", goal);
        if goal.left_iter().eq(goal.right_iter()) {
            goals.pop();
            println!("Solved! {} goals remains.", goals.len());
            continue;
        }
        let left_term_count = goal.left_iter().len() as isize;
        let right_term_count = goal.right_iter().len() as isize;
        let index = {
            if goal.is_left_ignorable() {
                if goal.is_right_ignorable() {
                    println!("Absurd! Exiting.");
                    break;
                } else {
                    1
                }
            } else if goal.is_right_ignorable() {
                -1
            } else {
                loop {
                    let index = prompt_selection("index", -left_term_count, right_term_count);
                    if index != 0 {
                        break index;
                    }
                }
            }
        };
        let mut extra_goal: Option<Sequent> = None;
        let new_goal = if index < 0 {
            let no_weakening = left_term_count == 1;
            match goal.focus_left(index.abs_diff(-1)) {
                LeftRule::None => None,
                LeftRule::Var { weakening } | LeftRule::Then { weakening } => {
                    if no_weakening {
                        None
                    } else {
                        Some(weakening())
                    }
                }
                LeftRule::Not { weakening, intro } => {
                    if no_weakening {
                        Some(intro())
                    } else {
                        match prompt_selection("(0)weakening? (1)intro?", 0, 1) {
                            0 => Some(weakening()),
                            1 => Some(intro()),
                            _ => None,
                        }
                    }
                }
                LeftRule::And {
                    weakening,
                    inl,
                    inr,
                } => {
                    if no_weakening {
                        match prompt_selection("(1)inl? (2)inr?", 1, 2) {
                            1 => Some(inl()),
                            2 => Some(inr()),
                            _ => None,
                        }
                    } else {
                        match prompt_selection("(0)weakening? (1)inl? (2)inr?", 0, 2) {
                            0 => Some(weakening()),
                            1 => Some(inl()),
                            2 => Some(inr()),
                            _ => None,
                        }
                    }
                }
                LeftRule::Or { weakening, intro } => {
                    let mut split = || {
                        let (left_goal, right_goal) = intro();
                        extra_goal = Some(right_goal);
                        println!("saved> {left_goal}");
                        Some(left_goal)
                    };
                    if no_weakening {
                        split()
                    } else {
                        match prompt_selection("(0)weakening? (1)intro?", 0, 1) {
                            0 => Some(weakening()),
                            1 => split(),
                            _ => None,
                        }
                    }
                }
            }
        } else {
            let no_weakening = right_term_count == 1;
            match goal.focus_right(index.abs_diff(1)) {
                RightRule::None => None,
                RightRule::Var { weakening } => {
                    if no_weakening {
                        None
                    } else {
                        Some(weakening())
                    }
                }
                RightRule::Not { weakening, intro } => {
                    if no_weakening {
                        Some(intro())
                    } else {
                        match prompt_selection("(0)weakening? (1)intro?", 0, 1) {
                            0 => Some(weakening()),
                            1 => Some(intro()),
                            _ => None,
                        }
                    }
                }
                RightRule::Or {
                    weakening,
                    inl,
                    inr,
                } => {
                    if no_weakening {
                        match prompt_selection("(1)inl? (2)inr?", 1, 2) {
                            1 => Some(inl()),
                            2 => Some(inr()),
                            _ => None,
                        }
                    } else {
                        match prompt_selection("(0)weakening? (1)inl? (2)inr?", 0, 2) {
                            0 => Some(weakening()),
                            1 => Some(inl()),
                            2 => Some(inr()),
                            _ => None,
                        }
                    }
                }
                RightRule::And { weakening, intro } => {
                    let mut split = || {
                        let (left_goal, right_goal) = intro();
                        extra_goal = Some(right_goal);
                        println!("saved> {left_goal}");
                        Some(left_goal)
                    };
                    if no_weakening {
                        split()
                    } else {
                        match prompt_selection("(0)weakening? (1)intro?", 0, 1) {
                            0 => Some(weakening()),
                            1 => split(),
                            _ => None,
                        }
                    }
                }
                RightRule::Then { weakening, intro } => {
                    if no_weakening {
                        Some(intro())
                    } else {
                        match prompt_selection("(0)weakening? (1)intro?", 0, 1) {
                            0 => Some(weakening()),
                            1 => Some(intro()),
                            _ => None,
                        }
                    }
                }
            }
        };
        if let Some(new_goal) = new_goal {
            *goal = new_goal;
        } else {
            println!("No actions!");
        }
        if let Some(extra_goal) = extra_goal {
            goals.push(extra_goal);
        }
    }
}

fn prompt_selection(prompt: &str, min_inclusive: isize, max_inclusive: isize) -> isize {
    loop {
        print!("{}> ", prompt);
        let mut line = String::new();
        io::stdout().flush().expect("Failed to flush!");
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line!");
        if let Ok(selection) = line.trim().parse::<isize>() {
            if min_inclusive <= selection && selection <= max_inclusive {
                return selection;
            }
        }
        println!(
            "Please enter an integer between {} and {} (inclusive)",
            min_inclusive, max_inclusive
        );
    }
}

pub mod kernel {
    #[derive(PartialEq)]
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

        pub fn is_absurd(&self) -> bool {
            self.left.is_empty() && self.right.is_empty()
        }

        pub fn focus_left(
            &self,
            index: usize,
        ) -> LeftRule<
            'a,
            impl Fn() -> Sequent<'a> + '_,
            impl Fn() -> Sequent<'a> + '_,
            impl Fn() -> (Sequent<'a>, Sequent<'a>) + '_,
            impl Fn() -> Sequent<'a> + '_,
            impl Fn() -> Sequent<'a> + '_,
        > {
            if let Some(focused_term) = self.left.get(index) {
                let weakening = move || {
                    let mut new = self.clone();
                    new.left.remove(index);
                    new
                };
                match focused_term {
                    Term::Var(_) => LeftRule::Var { weakening },
                    Term::Not(term) => LeftRule::Not {
                        weakening,
                        intro: move || {
                            let mut new = weakening();
                            new.right.push(term.clone());
                            new
                        },
                    },
                    Term::Or(left, right) => LeftRule::Or {
                        weakening,
                        intro: move || {
                            (
                                {
                                    let mut new = self.clone();
                                    new.left[index] = left.clone();
                                    new
                                },
                                {
                                    let mut new = self.clone();
                                    new.left[index] = right.clone();
                                    new
                                },
                            )
                        },
                    },
                    Term::And(left, right) => LeftRule::And {
                        weakening,
                        inl: move || {
                            let mut new = self.clone();
                            new.left[index] = left.clone();
                            new
                        },
                        inr: move || {
                            let mut new = self.clone();
                            new.left[index] = right.clone();
                            new
                        },
                    },
                    Term::Then(_, _) => LeftRule::Then { weakening },
                }
            } else {
                LeftRule::None
            }
        }

        pub fn focus_right(
            &self,
            index: usize,
        ) -> RightRule<
            'a,
            impl Fn() -> Sequent<'a> + '_,
            impl Fn() -> Sequent<'a> + '_,
            impl Fn() -> (Sequent<'a>, Sequent<'a>) + '_,
            impl Fn() -> Sequent<'a> + '_,
            impl Fn() -> Sequent<'a> + '_,
            impl Fn() -> Sequent<'a> + '_,
        > {
            if let Some(focused_term) = self.right.get(index) {
                let weakening = move || {
                    let mut new = self.clone();
                    new.right.remove(index);
                    new
                };
                match focused_term {
                    Term::Var(_) => RightRule::Var { weakening },
                    Term::Not(term) => RightRule::Not {
                        weakening,
                        intro: move || {
                            let mut new = weakening();
                            new.left.push(term.clone());
                            new
                        },
                    },
                    Term::And(left, right) => RightRule::And {
                        weakening,
                        intro: move || {
                            (
                                {
                                    let mut new = self.clone();
                                    new.right[index] = left.clone();
                                    new
                                },
                                {
                                    let mut new = self.clone();
                                    new.right[index] = right.clone();
                                    new
                                },
                            )
                        },
                    },
                    Term::Or(left, right) => RightRule::Or {
                        weakening,
                        inl: move || {
                            let mut new = self.clone();
                            new.right[index] = left.clone();
                            new
                        },
                        inr: move || {
                            let mut new = self.clone();
                            new.right[index] = right.clone();
                            new
                        },
                    },
                    Term::Then(left, right) => RightRule::Then {
                        weakening,
                        intro: move || {
                            let mut new = self.clone();
                            new.left.push(left.clone());
                            new.right[index] = right.clone();
                            new
                        },
                    },
                }
            } else {
                RightRule::None
            }
        }
    }

    pub enum LeftRule<
        'a,
        Weakening: Fn() -> Sequent<'a>,
        IntroInv: Fn() -> Sequent<'a>,
        IntroPair: Fn() -> (Sequent<'a>, Sequent<'a>),
        IntroLeft: Fn() -> Sequent<'a>,
        IntroRight: Fn() -> Sequent<'a>,
    > {
        None,
        Var {
            weakening: Weakening,
        },
        Not {
            weakening: Weakening,
            intro: IntroInv,
        },
        Or {
            weakening: Weakening,
            intro: IntroPair,
        },
        And {
            weakening: Weakening,
            inl: IntroLeft,
            inr: IntroRight,
        },
        Then {
            weakening: Weakening,
        },
    }
    pub enum RightRule<
        'a,
        Weakening: Fn() -> Sequent<'a>,
        IntroInv: Fn() -> Sequent<'a>,
        IntroPair: Fn() -> (Sequent<'a>, Sequent<'a>),
        IntroLeft: Fn() -> Sequent<'a>,
        IntroRight: Fn() -> Sequent<'a>,
        IntroCut: Fn() -> Sequent<'a>,
    > {
        None,
        Var {
            weakening: Weakening,
        },
        Not {
            weakening: Weakening,
            intro: IntroInv,
        },
        And {
            weakening: Weakening,
            intro: IntroPair,
        },
        Or {
            weakening: Weakening,
            inl: IntroLeft,
            inr: IntroRight,
        },
        Then {
            weakening: Weakening,
            intro: IntroCut,
        },
    }
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

fn superscript(n: isize, f: &mut fmt::Formatter) -> fmt::Result {
    if n < 0 {
        write!(f, "⁻")?;
    }
    let mut n = n.abs_diff(0);
    let mut digits = Vec::<usize>::new();
    while n != 0 {
        digits.push(n % 10);
        n /= 10;
    }
    for &digit in digits.iter().rev() {
        static DIGIT_MAP: &'static [char] = &['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
        write!(f, "{}", DIGIT_MAP[digit])?;
    }
    write!(f, "⁾")
}

fn display_terms(
    f: &mut fmt::Formatter,
    mut terms: std::slice::Iter<&Term>,
    mut index: isize,
) -> fmt::Result {
    let show_index = !is_ignorable(terms.clone());
    if let Some(&term) = terms.next() {
        if show_index {
            superscript(index, f)?;
        }
        term.fmt(f)?;
        for &term in terms {
            index += 1;
            superscript(index, f)?;
            write!(f, ", {}", term)?;
        }
    } else {
        write!(f, "•")?;
    }
    Ok(())
}

fn is_ignorable(mut iter: std::slice::Iter<&Term>) -> bool {
    if let Some(term) = iter.next() {
        if let Term::Var(_) = term {
            if let None = iter.next() {
                return true;
            }
        }
    } else {
        return true;
    }
    return false;
}

impl fmt::Display for Sequent<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        display_terms(f, self.left_iter(), -(self.left_iter().len() as isize))?;
        write!(f, " ⊢ ")?;
        display_terms(f, self.right_iter(), 1)
    }
}

impl Sequent<'_> {
    pub fn is_left_ignorable(&self) -> bool {
        is_ignorable(self.left_iter())
    }

    pub fn is_right_ignorable(&self) -> bool {
        is_ignorable(self.right_iter())
    }
}
