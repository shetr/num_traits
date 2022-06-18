use std::mem;

pub trait FixedNumBounds {
    fn min() -> Self;
    fn max() -> Self;
}

pub trait BinaryBounds: Sized {
    fn binary_min() -> Self;
    fn binary_max() -> Self;
    fn binary_lower_half() -> Self;
    fn binary_upper_half() -> Self;
    fn bytes_size() -> usize { mem::size_of::<Self>() }
}

// smallest number greather than zero
pub trait Epsilon {
    fn epsilon() -> Self;
}

pub trait InfBounds {
    fn inf() -> Self;
    fn minf() -> Self;
}

pub trait NotANumber {
    fn nan() -> Self;
}

pub trait FloatSpecialValues:
    Epsilon + InfBounds + NotANumber
{}

pub trait FixedFloatBounds {
    fn non_inf_min() -> Self;
    fn non_inf_max() -> Self;
}

impl FixedNumBounds for u8 {
    fn min() -> Self { u8::MIN }
    fn max() -> Self { u8::MAX }
}

impl BinaryBounds for u8 {
    fn binary_min() -> Self { u8::MIN }
    fn binary_max() -> Self { u8::MAX }
    fn binary_lower_half() -> Self { u8::MAX >> (Self::bytes_size()*4) }
    fn binary_upper_half() -> Self { u8::MAX << (Self::bytes_size()*4) }
}