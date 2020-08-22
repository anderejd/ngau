use crate::f32::Rad;
use std::f32;
use std::fmt;
use std::ops::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An angle, in degrees.
#[derive(Default, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Deg(f32);

impl Deg {
    /// `f` is expected to represent degrees, no conversion is performed and the
    /// float is simply wrapped by this "newtype".
    pub fn new(f: f32) -> Self {
        Self(f)
    }

    /// Get the contained f32 value.
    pub fn value(self) -> f32 {
        self.0
    }
}

impl From<Rad> for Deg {
    fn from(rad: Rad) -> Deg {
        Deg(rad.value() * (180.0 / f32::consts::PI))
    }
}

impl Neg for Deg {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Add for Deg {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Deg {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Deg {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Deg {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

/// Dividing an angle by another angle gives a ratio.
impl Div for Deg {
    type Output = f32;

    fn div(self, rhs: Self) -> f32 {
        self.0 / rhs.0
    }
}

/// Dividing an angle by a number gives a new and scaled angle.
impl Div<f32> for Deg {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self(self.0 / rhs)
    }
}

impl DivAssign<f32> for Deg {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs
    }
}

/// Multiplying an angle with a number gives a new and scaled angle.
/// Multiplying an angle by another angle probably doesn't make any sense(?).
impl Mul<f32> for Deg {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs)
    }
}

impl MulAssign<f32> for Deg {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs
    }
}

/// Since Div is implemented, this seems to make sense.
impl Rem for Deg {
    type Output = f32;

    fn rem(self, rhs: Self) -> f32 {
        self.0 % rhs.0
    }
}

/// Since Div<f32> is implemented, this seems to make sense.
impl Rem<f32> for Deg {
    type Output = Self;

    fn rem(self, rhs: f32) -> Self {
        Self(self.0 % rhs)
    }
}

/// Since DivAssign<f32> is implemented, this seems to make sense.
impl RemAssign<f32> for Deg {
    fn rem_assign(&mut self, rhs: f32) {
        self.0 %= rhs;
    }
}

impl fmt::Debug for Deg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}°", self.0)
    }
}

impl fmt::Display for Deg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Just forward to Debug.
        fmt::Debug::fmt(self, f)
    }
}
