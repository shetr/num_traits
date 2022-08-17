use std::ops::*;

pub trait BinaryOpsSized:
    BitAnd + BitOr + BitXor +
    BitAndAssign + BitOrAssign + BitXorAssign +
    Sized
{}

pub trait BinaryOpsGeneral:
    Not +
    Shl<usize> + Shr<usize> +
    ShlAssign<usize> + ShrAssign<usize>
{}

pub trait BinaryOpsRef<Rhs>:
    BitAnd<Rhs> + BitOr<Rhs> + BitXor<Rhs> +
    BitAndAssign<Rhs> + BitOrAssign<Rhs> + BitXorAssign<Rhs>
{}
/*
pub trait BinaryOps<'a>:
    BinaryOpsGeneral + BinaryOpsRef<&'a Self>
{}*/

impl BinaryOpsSized for u8 {}
