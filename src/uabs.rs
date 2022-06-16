
// absolute value, implemented even for unsigned types (just returns the value)
pub trait UAbs {
    fn uabs(&self) -> Self;
}

impl UAbs for u8 {
    fn uabs(&self) -> u8 {
        self.clone()
    }
}
impl UAbs for u16 {
    fn uabs(&self) -> u16 {
        self.clone()
    }
}
impl UAbs for u32 {
    fn uabs(&self) -> u32 {
        self.clone()
    }
}
impl UAbs for u64 {
    fn uabs(&self) -> u64 {
        self.clone()
    }
}
impl UAbs for u128 {
    fn uabs(&self) -> u128 {
        self.clone()
    }
}
impl UAbs for usize {
    fn uabs(&self) -> usize {
        self.clone()
    }
}

impl UAbs for i8 {
    fn uabs(&self) -> i8 {
        self.abs()
    }
}
impl UAbs for i16 {
    fn uabs(&self) -> i16 {
        self.abs()
    }
}
impl UAbs for i32 {
    fn uabs(&self) -> i32 {
        self.abs()
    }
}
impl UAbs for i64 {
    fn uabs(&self) -> i64 {
        self.abs()
    }
}
impl UAbs for i128 {
    fn uabs(&self) -> i128 {
        self.abs()
    }
}
impl UAbs for isize {
    fn uabs(&self) -> isize {
        self.abs()
    }
}
impl UAbs for f32 {
    fn uabs(&self) -> f32 {
        self.abs()
    }
}
impl UAbs for f64 {
    fn uabs(&self) -> f64 {
        self.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uabs() {
        assert_eq!((1u32).uabs(), 1u32);
        assert_eq!((1i32).uabs(), 1i32);
        assert_eq!((-1i32).uabs(), 1i32);
        assert_eq!((1f32).uabs(), 1f32);
        assert_eq!((-1f32).uabs(), 1f32);
    }
}