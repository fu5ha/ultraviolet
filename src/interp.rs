//! Interpolation on types for which it makes sense.
use crate::*;

/// Pure linear interpolation, i.e. `(1.0 - t) * self + (t) * end`.
///
/// For interpolating `Rotor`s with linear interpolation, you almost certainly
/// want to normalize the returned `Rotor`. For example,
/// ```rs
/// let interpolated_rotor = rotor1.lerp(rotor2, 0.5).normalized();
/// ```
/// For most cases (especially where perfomrance is the primary concern, like in
/// animation interpolation for games, this 'normalized lerp' or 'nlerp' is probably
/// what you want to use. However, there are situations in which you really want
/// the interpolation between two `Rotor`s to be of constant angular velocity. In this
/// case, check out `Slerp`.
pub trait Lerp<T> {
    fn lerp(&self, end: Self, t: T) -> Self;
}

macro_rules! impl_lerp {
    ($($tt:ident => ($($vt:ident),+)),+) => {
        $($(impl Lerp<$tt> for $vt {
            /// Linearly interpolate between `self` and `end` by `t` between 0.0 and 1.0.
            /// i.e. `(1.0 - t) * self + (t) * end`.
            ///
            /// For interpolating `Rotor`s with linear interpolation, you almost certainly
            /// want to normalize the returned `Rotor`. For example,
            /// ```rs
            /// let interpolated_rotor = rotor1.lerp(rotor2, 0.5).normalized();
            /// ```
            /// For most cases (especially where perfomrance is the primary concern, like in
            /// animation interpolation for games, this 'normalized lerp' or 'nlerp' is probably
            /// what you want to use. However, there are situations in which you really want
            /// the interpolation between two `Rotor`s to be of constant angular velocity. In this
            /// case, check out `Slerp`.
            #[inline]
            fn lerp(&self, end: Self, t: $tt) -> Self {
                *self * ($tt::splat(1.0) - t) + end * t
            }
        })+)+
    };
}

impl_lerp!(
    f32 => (Vec2, Vec3, Vec4, Bivec2, Bivec3, Rotor2, Rotor3),
    f32x4 => (Vec2x4, Vec3x4, Vec4x4, Bivec2x4, Bivec3x4, Rotor2x4, Rotor3x4));

/// Spherical-linear interpolation.
///
/// Basically, interpolation that maintains a constant angular velocity
/// from one orientation on a unit hypersphere to another. This is sorta the "high quality" interpolation
/// for `Rotor`s, and it can also be used to interpolate other things, one example being interpolation of
/// 3d normal vectors.
///
/// Note that you should often normalize the result returned by this operation, when working with `Rotor`s, etc!
pub trait Slerp<T> {
    fn slerp(&self, end: Self, t: T) -> Self;
}

macro_rules! impl_slerp_rotor3 {
    ($($tt:ident => ($($vt:ident),+)),+) => {
        $($(impl Slerp<$tt> for $vt {
            /// Spherical-linear interpolation between `self` and `end` based on `t` from 0.0 to 1.0.
            ///
            /// `self` and `end` should both be normalized or something bad will happen!
            ///
            /// Basically, interpolation that maintains a constant angular velocity
            /// from one orientation on a unit hypersphere to another. This is sorta the "high quality" interpolation
            /// for `Rotor`s, and it can also be used to interpolate other things, one example being interpolation of
            /// 3d normal vectors.
            ///
            /// Note that you should often normalize the result returned by this operation, when working with `Rotor`s, etc!
            #[inline]
            fn slerp(&self, end: Self, t: $tt) -> Self {
                let dot = self.dot(end);

                if dot > 0.9995 {
                    return self.lerp(end, t);
                }

                let dot = dot.min(1.0).max(-1.0);

                let theta_0 = dot.acos(); // angle between inputs
                let theta = theta_0 * t; // amount of said angle to travel

                let v2 = (end - (*self * dot)).normalized(); // create orthonormal basis between self and `v2`

                let (s, c) = theta.sin_cos();

                let mut n = *self;

                n.s = c.mul_add(self.s, s * v2.s);
                n.bv.xy = c.mul_add(self.bv.xy, s * v2.bv.xy);
                n.bv.xz = c.mul_add(self.bv.xz, s * v2.bv.xz);
                n.bv.yz = c.mul_add(self.bv.yz, s * v2.bv.yz);

                n
            }
        })+)+
    };
}

impl_slerp_rotor3!(
    f32 => (Rotor3));

macro_rules! impl_slerp_rotor3_wide {
    ($($tt:ident => ($($vt:ident),+)),+) => {
        $($(impl Slerp<$tt> for $vt {
            /// Spherical-linear interpolation between `self` and `end` based on `t` from 0.0 to 1.0.
            ///
            /// `self` and `end` should both be normalized or something bad will happen!
            ///
            /// The implementation for SIMD types also requires that the two things being interpolated between
            /// are not exactly aligned, or else the result is undefined.
            ///
            /// Basically, interpolation that maintains a constant angular velocity
            /// from one orientation on a unit hypersphere to another. This is sorta the "high quality" interpolation
            /// for `Rotor`s, and it can also be used to interpolate other things, one example being interpolation of
            /// 3d normal vectors.
            ///
            /// Note that you should often normalize the result returned by this operation, when working with `Rotor`s, etc!
            #[inline]
            fn slerp(&self, end: Self, t: $tt) -> Self {
                let dot = self.dot(end);

                let dot = dot.min($tt::splat(1.0)).max($tt::splat(-1.0));

                let theta_0 = dot.acos(); // angle between inputs
                let theta = theta_0 * t; // amount of said angle to travel

                let v2 = (end - (*self * dot)).normalized(); // create orthonormal basis between self and `v2`

                let (s, c) = theta.sin_cos();

                let mut n = *self;

                n.s = c.mul_add(self.s, s * v2.s);
                n.bv.xy = c.mul_add(self.bv.xy, s * v2.bv.xy);
                n.bv.xz = c.mul_add(self.bv.xz, s * v2.bv.xz);
                n.bv.yz = c.mul_add(self.bv.yz, s * v2.bv.yz);

                n
            }
        })+)+
    };
}

impl_slerp_rotor3_wide!(
    f32x4 => (Rotor3x4));

macro_rules! impl_slerp_gen {
    ($($tt:ident => ($($vt:ident),+)),+) => {
        $($(impl Slerp<$tt> for $vt {
            /// Spherical-linear interpolation between `self` and `end` based on `t` from 0.0 to 1.0.
            ///
            /// `self` and `end` should both be normalized or something bad will happen!
            ///
            /// The implementation for SIMD types also requires that the two things being interpolated between
            /// are not exactly aligned, or else the result is undefined.
            ///
            /// Basically, interpolation that maintains a constant angular velocity
            /// from one orientation on a unit hypersphere to another. This is sorta the "high quality" interpolation
            /// for `Rotor`s, and it can also be used to interpolate other things, one example being interpolation of
            /// 3d normal vectors.
            ///
            /// Note that you should often normalize the result returned by this operation, when working with `Rotor`s, etc!
            #[inline]
            fn slerp(&self, end: Self, t: $tt) -> Self {
                let dot = self.dot(end);

                let dot = dot.min($tt::splat(1.0)).max($tt::splat(-1.0));

                let theta_0 = dot.acos(); // angle between inputs
                let theta = theta_0 * t; // amount of said angle to travel

                let v2 = (end - (*self * dot)).normalized(); // create orthonormal basis between self and `v2`

                let (s, c) = theta.sin_cos();

                *self * c + v2 * s
            }
        })+)+
    };
}

impl_slerp_gen!(
    f32 => (Vec2, Vec3, Vec4, Bivec2, Bivec3, Rotor2),
    f32x4 => (Vec2x4, Vec3x4, Vec4x4, Bivec2x4, Bivec3x4, Rotor2x4));
