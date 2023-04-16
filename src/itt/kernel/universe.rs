pub struct Universe(usize);
impl Universe {
    fn level(&self) -> usize {
        self.0
    }
    fn succ(&self) -> Self {
        Self(self.0 + 1)
    }
    fn max(u: &Universe, v: &Universe) -> Universe {
        Self(core::cmp::max(u.0, v.0))
    }
    fn imax(u: &Universe, v: &Universe) -> Universe {
        if v.0 == 0 {
            Self(0)
        } else {
            Self::max(u, v)
        }
    }
}
impl super::Term<Universe> for Universe {
    fn r#type(&self) -> Self {
        self.succ()
    }
}
