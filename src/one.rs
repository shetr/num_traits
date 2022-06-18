
pub trait One {
    fn one() -> Self;
}

impl One for u8 {
    fn one() -> u8 {
        1u8
    }
}
impl One for u16 {
    fn one() -> u16 {
        1u16
    }
}
impl One for u32 {
    fn one() -> u32 {
        1u32
    }
}
impl One for u64 {
    fn one() -> u64 {
        1u64
    }
}
impl One for u128 {
    fn one() -> u128 {
        1u128
    }
}
impl One for usize {
    fn one() -> usize {
        1usize
    }
}
impl One for i8 {
    fn one() -> i8 {
        1i8
    }
}
impl One for i16 {
    fn one() -> i16 {
        1i16
    }
}
impl One for i32 {
    fn one() -> i32 {
        1i32
    }
}
impl One for i64 {
    fn one() -> i64 {
        1i64
    }
}
impl One for i128 {
    fn one() -> i128 {
        1i128
    }
}
impl One for isize {
    fn one() -> isize {
        1isize
    }
}
impl One for f32 {
    fn one() -> f32 {
        1f32
    }
}
impl One for f64 {
    fn one() -> f64 {
        1f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_one<T: One>() -> T {
        T::one()
    }

    #[test]
    fn one() {
        assert_eq!(i32::one(), 1i32);
    }

    // try with some template function
    #[test]
    fn one_template() {
        assert_eq!(get_one::<i32>(), 1i32);
    }
}