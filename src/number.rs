use std::ops::*;
use std::cmp::*;
use std::fmt::*;
use std::hash::Hash;
use zero::Zero;
use one::One;
use uabs::UAbs;

// get some inspiration from Julia
// what about Neg? it shouldn't be in unsigned nubers
// think about the conversion operators
// FromStr, FloatToInt, From / Into
// conversion to some generic base for some types (probably only integers)
// std::iter::Step for integers

pub trait Number:  
    Clone + Hash + Eq + Ord + Neg +
    Add + Sub + Mul + Div + 
    AddAssign + SubAssign + MulAssign + DivAssign +
    Binary + Debug + Display + LowerExp + UpperExp + LowerHex + UpperHex + Octal +
    Zero + One + UAbs
{}

pub trait BinaryOps:
    BitAnd + BitOr + BitXor + Shl<usize> + Shr<usize> +
    BitAndAssign + BitOrAssign + BitXorAssign + ShlAssign<usize> + ShrAssign<usize>
{}

pub trait Integer:
    Number + BinaryOps + Rem + RemAssign
{}

// TODO: rethink how to handle signed/unsigned
//

pub trait SInteger:
    Integer
{}

pub trait UInteger:
    Integer
{}

// primitive type trait shloud be implemented only for primitive types

pub trait SInt:
    Copy + SInteger
{}

pub trait UInt:
    Copy + UInteger
{}
