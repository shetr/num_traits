use std::ops::*;
use std::cmp::*;
use std::fmt::*;
// when it's not nightly: use std::iter::Step;
use std::hash::Hash;
use std::str::FromStr;
use crate::*;

// get some inspiration from Julia
// what about Neg? it shouldn't be in unsigned numbers
// think about the conversion operators
// FromStr, FloatToInt, From / Into
// conversion to some generic base for some types (probably only integers)
// std::iter::Step for integers
// trait for getting maximum, minimum, different for floats, ints, fixed
// for all fixed get max and min value
// for all fixed ints get also max/min bit value and left, right masks
// for all floats get inf, -inf
// for all fixed floats get max/min non-inf value, epsilon
// for all fixed rational: max, min, epsilon
// also geting binary size, both for fixed and dynamic numbers, but divide it to different traits

pub trait Number:
    Clone + Hash + Eq + Ord +
    Add + Sub + Mul + Div +
    AddAssign + SubAssign + MulAssign + DivAssign +
    Binary + Debug + Display + LowerExp + UpperExp + LowerHex + UpperHex + Octal +
    FromStr +
    Zero + One + UAbs
{}

// FixedNum trait is probably enough
//pub trait Dynamic:
//{}

pub trait FixedNum:
    FixedNumBounds + Sized
{}

pub trait Integer:
    Number /*+ BinaryOps*/ + Rem + RemAssign
{}

// TODO: rethink how to handle signed/unsigned
// I think it's fine to divide to 2 traits - when I have template class which has both, I can implement selectively

pub trait UInteger:
    Integer
{}

pub trait SInteger:
    Integer + Neg
{}

// primitive type trait shloud be implemented only for primitive types

pub trait FixedInt:
    Copy + FixedNum + BinaryBounds
{}

pub trait UInt:
    UInteger + FixedInt
{}

pub trait SInt:
    SInteger + FixedInt
{}


pub trait Rational:
    Number + Neg
{}

pub trait FixedRational:
    Copy + FixedNum + Epsilon
{}

pub trait AbstractFloat:
    Rational + FloatSpecialValues
{}

pub trait FixedFloat:
    Copy + FixedNum + FixedFloatBounds
{}

pub trait Float:
    AbstractFloat + FixedFloat
{}


impl Number for u8 {}
//impl BinaryOps for u8 {}
impl FixedNum for u8 {}
impl Integer for u8 {}
impl UInteger for u8 {}
impl FixedInt for u8 {}
impl UInt for u8 {}

/* 
impl Number for f32 {}
impl FixedNum for f32 {}
impl Rational for f32 {}
impl AbstractFloat for f32 {}
impl FixedFloat for f32 {}
impl Float for f32 {}*/

#[cfg(test)]
mod tests {
    use super::*;

    fn get_one<T: Number>() -> T {
        T::one()
    }

    // try with some template function
    #[test]
    fn one_template_int() {
        assert_eq!(get_one::<u8>(), 1u8);
    }

    // try with some template function
    /*#[test]
    fn one_template_float() {
        assert_eq!(get_one::<f32>(), 1f32);
    }*/
}