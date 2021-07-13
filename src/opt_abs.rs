
pub trait OptAbs {
    fn opt_abs(&self) -> Self;
}

impl OptAbs for u8 {
    fn opt_abs(&self) -> u8 {
        self.clone()
    }
}
impl OptAbs for u16 {
    fn opt_abs(&self) -> u16 {
        self.clone()
    }
}
impl OptAbs for u32 {
    fn opt_abs(&self) -> u32 {
        self.clone()
    }
}
impl OptAbs for u64 {
    fn opt_abs(&self) -> u64 {
        self.clone()
    }
}
impl OptAbs for u128 {
    fn opt_abs(&self) -> u128 {
        self.clone()
    }
}
impl OptAbs for usize {
    fn opt_abs(&self) -> usize {
        self.clone()
    }
}

impl OptAbs for i8 {
    fn opt_abs(&self) -> i8 {
        self.abs()
    }
}
impl OptAbs for i16 {
    fn opt_abs(&self) -> i16 {
        self.abs()
    }
}
impl OptAbs for i32 {
    fn opt_abs(&self) -> i32 {
        self.abs()
    }
}
impl OptAbs for i64 {
    fn opt_abs(&self) -> i64 {
        self.abs()
    }
}
impl OptAbs for i128 {
    fn opt_abs(&self) -> i128 {
        self.abs()
    }
}
impl OptAbs for isize {
    fn opt_abs(&self) -> isize {
        self.abs()
    }
}
impl OptAbs for f32 {
    fn opt_abs(&self) -> f32 {
        self.abs()
    }
}
impl OptAbs for f64 {
    fn opt_abs(&self) -> f64 {
        self.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opt_abs() {
        assert_eq!((1u32).opt_abs(), 1u32);
        assert_eq!((1i32).opt_abs(), 1i32);
        assert_eq!((-1i32).opt_abs(), 1i32);
        assert_eq!((1f32).opt_abs(), 1f32);
        assert_eq!((-1f32).opt_abs(), 1f32);
    }
}