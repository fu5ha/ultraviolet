use wide::f32x4;

use crate::bivec::*;
use crate::rotor::*;
use crate::vec::*;

pub trait Lerp<T> {
    fn lerp(&self, end: Self, t: T) -> Self;
}

macro_rules! impl_lerp {
    ($($tt:ident => ($($vt:ident),+)),+) => {
        $($(impl Lerp<$tt> for $vt {
            #[inline]
            fn lerp(&self, end: Self, t: $tt) -> Self {
                *self * ($tt::from(1.0) - t) + end * t
            }
        })+)+
    };
}

impl_lerp!(
    f32 => (Vec2, Vec3, Vec4, Bivec2, Bivec3, Rotor2, Rotor3),
    f32x4 => (Wec2, Wec3, Wec4, WBivec2, WBivec3, WRotor2, WRotor3));
