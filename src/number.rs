use std::ops::*;
use std::cmp::*;
use zero::Zero;
use one::One;

pub trait Number:  
    Clone + Eq + Ord + Neg +
    Add + Sub + Mul + Div + 
    AddAssign + SubAssign + MulAssign + DivAssign +
    Zero + One
{}

pub trait BinaryOps:
    BitAnd + BitOr + BitXor + Shl<usize> + Shr<usize> +
    BitAndAssign + BitOrAssign + BitXorAssign + ShlAssign<usize> + ShrAssign<usize>
{}

pub trait Integer:
    Number + BinaryOps + Rem + RemAssign
{}

// TODO: rethink how to handle signed/unsigned

pub trait SInteger:
    Integer
{}

pub trait UInteger:
    Integer
{}

pub trait SInt:
    Copy + SInteger
{}

pub trait UInt:
    Copy + UInteger
{}
