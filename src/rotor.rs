//! A rotor can be thought of in multiple ways, the first of which
//! is that it is the result of the 'geometric product' of two vectors,
//! denoted for two vectors `u` and `v` as simply `uv`. This operation is
//! defined as
//!
//! ```
//! uv = u · v + u ∧ v
//! ```
//!
//! As can be seen, this operation results in the addition of two different
//! types of values: first, the dot product will result in a scalar, and second,
//! the outer product will result in a bivector. The addition of these two different
//! types is not defined, but can be understood in a similar way as complex numbers,
//! i.e. as a 'bundle' of two different kinds of values.
//!
//! The reason we call this type of value a 'rotor' is that if you both left- and
//! right-multiply (using the geometric product) a rotor with a vector , you will
//! rotate the sandwiched vector. For example, if you start with two vectors,
//! `a` and `b`, and create a rotor `ab` from them, then rotate a vector `u` with this
//! rotor by doing `ba u ab`, you will end up rotating the vector `u` by in the plane
//! that corresponds to `a ∧ b` (i.e. the plane which is parallel with both vectors), by
//! twice the angle between `a` and `b`.

use crate::bivec::*;
use crate::vec::*;
use wide::f32x4;

use std::ops::*;

macro_rules! rotor2s {
    ($($rn:ident => ($vt:ident, $bt:ident, $t:ident)),+) => {
        $(
        /// A Rotor in 2d space.
        #[derive(Clone, Copy, Debug)]
        pub struct $rn {
            pub s: $t,
            pub bv: $bt,
        }

        impl $rn {
            #[inline]
            pub fn new(scalar: $t, bivector: $bt) -> Self {
                Self {
                    s: scalar,
                    bv: bivector,
                }
            }

            #[inline]
            pub fn mag_sq(&self) -> $t {
                self.s * self.s + self.bv.xy * self.bv.xy
            }

            #[inline]
            pub fn mag(&self) -> $t {
                self.mag_sq().sqrt()
            }

            #[inline]
            pub fn normalize(&mut self) {
                let mag = self.mag();
                self.s /= mag;
                self.bv.xy /= mag;
            }

            #[inline]
            pub fn normalized(&self) -> Self {
                let mut s = *self;
                s.normalize();
                s
            }

            #[inline]
            pub fn reverse(&mut self) {
                self.bv = -self.bv;
            }

            #[inline]
            pub fn reversed(&self) -> Self {
                let mut s = *self;
                s.reverse();
                s
            }
        }

        impl Mul for $rn {
            type Output = Self;
            #[inline]
            fn mul(mut self, rhs: Self) -> Self {
                self *= rhs;
                self
            }
        }

        impl MulAssign for $rn {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                self.s = self.s * rhs.s - self.bv.xy * rhs.bv.xy;
                self.bv.xy = self.s * rhs.bv.xy + rhs.s * self.bv.xy;
            }
        }

        impl Mul<$vt> for $rn {
            type Output = $vt;
            #[inline]
            fn mul(self, mut rhs: $vt) -> $vt {
                let s2xy2 = self.s * self.s + self.bv.xy * self.bv.xy;
                rhs.x = s2xy2 * rhs.x + ($t::from(1.0) - self.s) * self.bv.xy * rhs.y;
                rhs.y = s2xy2 * rhs.y;
                rhs
            }
        }
        )+
    }
}

rotor2s!(Rotor2 => (Vec2, Bivec2, f32), WRotor2 => (Wec2, WBivec2, f32x4));

macro_rules! rotor3s {
    ($($rn:ident => ($vt:ident, $bt:ident, $t:ident)),+) => {
        $(
        /// A Rotor in 3d space.
        #[derive(Clone, Copy, Debug)]
        pub struct $rn {
            pub s: $t,
            pub bv: $bt,
        }

        impl $rn {
            #[inline]
            pub fn new(scalar: $t, bivector: $bt) -> Self {
                Self {
                    s: scalar,
                    bv: bivector,
                }
            }

            #[inline]
            pub fn mag_sq(&self) -> $t {
                self.s * self.s + self.bv.xy * self.bv.xy + self.bv.xz * self.bv.xz + self.bv.yz * self.bv.yz
            }

            #[inline]
            pub fn mag(&self) -> $t {
                self.mag_sq().sqrt()
            }

            #[inline]
            pub fn normalize(&mut self) {
                let mag = self.mag();
                self.s /= mag;
                self.bv.xy /= mag;
                self.bv.xz /= mag;
                self.bv.yz /= mag;
            }

            #[inline]
            pub fn normalized(&self) -> Self {
                let mut s = *self;
                s.normalize();
                s
            }

            #[inline]
            pub fn reverse(&mut self) {
                self.bv = -self.bv;
            }

            #[inline]
            pub fn reversed(&self) -> Self {
                let mut s = *self;
                s.reverse();
                s
            }
        }

        impl Mul for $rn {
            type Output = Self;
            #[inline]
            fn mul(mut self, rhs: Self) -> Self {
                self *= rhs;
                self
            }
        }

        impl MulAssign for $rn {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                self.s = self.s * rhs.s
                    - self.bv.xy * rhs.bv.xy
                    - self.bv.xz * rhs.bv.xz
                    - self.bv.yz * rhs.bv.xz;

                self.bv.xy = self.s * rhs.bv.xy
                    + self.bv.xy * rhs.s
                    + self.bv.yz * rhs.bv.xz
                    - self.bv.xz * rhs.bv.yz;

                self.bv.xz = self.s * rhs.bv.xz
                    + self.bv.xz * rhs.s
                    - self.bv.yz * rhs.bv.xy
                    + self.bv.xy * rhs.bv.yz;

                self.bv.yz = self.s * rhs.bv.yz
                    + self.bv.yz * rhs.s
                    + self.bv.xz * rhs.bv.xy
                    - self.bv.xy * rhs.bv.xz;
            }
        }

        impl Mul<$vt> for $rn {
            type Output = $vt;
            #[inline]
            fn mul(self, mut rhs: $vt) -> $vt {
                // TODO
                rhs
            }
        }
        )+
    }
}

rotor3s!(Rotor3 => (Vec3, Bivec3, f32), WRotor3 => (Wec3, WBivec3, f32x4));
