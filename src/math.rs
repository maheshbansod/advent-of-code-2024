use std::ops::{Div, Rem, Sub};

use num::{One, Zero};

/// Extended euclidian algorithm for finding gcd
/// and a particular solution of adiophantine equation.
/// It returns (g, x0, y0)
/// **NOTE: It is assumed that a > b**
pub fn egcd<T>(a: T, b: T) -> (T, T, T)
where
    T: Div<Output = T>
        + std::cmp::PartialEq<T>
        + Rem<Output = T>
        + Zero
        + One
        + Sub<Output = T>
        + Copy,
{
    // if a < b {
    //     return egcd(b, a);
    // }
    if b == T::zero() {
        (a, T::one(), T::zero())
    } else {
        let (d, x, y) = egcd(b, a % b);
        (d, y, x - y * (a / b))
    }
}
