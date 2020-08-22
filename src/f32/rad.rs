use crate::f32::Deg;
use std::f32;
use std::fmt;
use std::ops::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An angle, in radians.
#[derive(Default, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rad(f32);

impl Rad {
    /// `f` is expected to represent radian units.
    pub fn new(f: f32) -> Self {
        Self(f)
    }

    /// Get the contained f32 value.
    pub fn value(self) -> f32 {
        self.0
    }

    pub fn full_turn() -> Self {
        // TODO: use this when it lands on stable:
        // https://doc.rust-lang.org/std/f32/consts/constant.TAU.html
        Self(f32::consts::PI * 2.0)
    }

    pub fn sin(f: f32) -> Self {
        Rad(f.sin())
    }

    pub fn cos(f: f32) -> Self {
        Rad(f.cos())
    }

    pub fn tan(f: f32) -> Self {
        Rad(f.tan())
    }

    pub fn sin_cos(f: f32) -> (Self, Self) {
        let t = f.sin_cos();
        (Self(t.0), Self(t.1))
    }

    pub fn asin(f: f32) -> Self {
        Rad(f.asin())
    }

    pub fn acos(f: f32) -> Self {
        Rad(f.acos())
    }

    pub fn atan(f: f32) -> Self {
        Rad(f.atan())
    }

    pub fn atan2(a: f32, b: f32) -> Self {
        Rad(a.atan2(b))
    }
}

impl From<Deg> for Rad {
    fn from(deg: Deg) -> Rad {
        Rad(deg.value() * (f32::consts::PI / 180.0))
    }
}

impl Neg for Rad {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Add for Rad {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Rad {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Rad {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Rad {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

/// Dividing an angle by another angle gives a ratio.
impl Div for Rad {
    type Output = f32;

    fn div(self, rhs: Self) -> f32 {
        self.0 / rhs.0
    }
}

/// Dividing an angle by a number gives a new and scaled angle.
impl Div<f32> for Rad {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self(self.0 / rhs)
    }
}

impl DivAssign<f32> for Rad {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs
    }
}

/// Multiplying an angle with a number gives a new and scaled angle.
/// Multiplying an angle by another angle probably doesn't make any sense(?).
impl Mul<f32> for Rad {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs)
    }
}

impl MulAssign<f32> for Rad {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs
    }
}

/// Since Div is implemented, this seems to make sense.
impl Rem for Rad {
    type Output = f32;

    fn rem(self, rhs: Self) -> f32 {
        self.0 % rhs.0
    }
}

/// Since Div<f32> is implemented, this seems to make sense.
impl Rem<f32> for Rad {
    type Output = Self;

    fn rem(self, rhs: f32) -> Self {
        Self(self.0 % rhs)
    }
}

/// Since DivAssign<f32> is implemented, this seems to make sense.
impl RemAssign<f32> for Rad {
    fn rem_assign(&mut self, rhs: f32) {
        self.0 %= rhs;
    }
}

impl fmt::Debug for Rad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} rad", self.0)
    }
}

impl fmt::Display for Rad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Just forward to Debug.
        fmt::Debug::fmt(self, f)
    }
}
