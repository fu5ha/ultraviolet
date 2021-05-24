//! Interpolation on types for which it makes sense.
use crate::*;

/// Pure linear interpolation, i.e. `(1.0 - t) * self + (t) * end`.
///
/// For interpolating `Rotor`s with linear interpolation, you almost certainly
/// want to normalize the returned `Rotor`. For example,
/// ```rs
/// let interpolated_rotor = rotor1.lerp(rotor2, 0.5).normalized();
/// ```
/// For most cases (especially where performance is the primary concern, like in
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
            /// For most cases (especially where performance is the primary concern, like in
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
    f32 => (f32, Vec2, Vec3, Vec4, Bivec2, Bivec3, Rotor2, Rotor3),
    f32x4 => (f32x4, Vec2x4, Vec3x4, Vec4x4, Bivec2x4, Bivec3x4, Rotor2x4, Rotor3x4),
    f32x8 => (f32x8, Vec2x8, Vec3x8, Vec4x8, Bivec2x8, Bivec3x8, Rotor2x8, Rotor3x8)
);

#[cfg(feature = "f64")]
impl_lerp!(
    f64 => (f64, DVec2, DVec3, DVec4, DBivec2, DBivec3, DRotor2, DRotor3),
    f64x2 => (f64x2, DVec2x2, DVec3x2, DVec4x2, DBivec2x2, DBivec3x2, DRotor2x2, DRotor3x2),
    f64x4 => (f64x4, DVec2x4, DVec3x4, DVec4x4, DBivec2x4, DBivec3x4, DRotor2x4, DRotor3x4)
);

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

                n.s = (c * self.s) + (s * v2.s);
                n.bv.xy = (c * self.bv.xy) + (s * v2.bv.xy);
                n.bv.zx = (c * self.bv.zx) + (s * v2.bv.zx);
                n.bv.yz = (c * self.bv.yz) + (s * v2.bv.yz);

                n
            }
        })+)+
    };
}

impl_slerp_rotor3!(
    f32 => (Rotor3)
);

#[cfg(feature = "f64")]
impl_slerp_rotor3!(
    f64 => (DRotor3)
);

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

                n.s = (c * self.s) + (s * v2.s);
                n.bv.xy = (c * self.bv.xy) + (s * v2.bv.xy);
                n.bv.zx = (c * self.bv.zx) + (s * v2.bv.zx);
                n.bv.yz = (c * self.bv.yz) + (s * v2.bv.yz);

                n
            }
        })+)+
    };
}

impl_slerp_rotor3_wide!(
    f32x4 => (Rotor3x4),
    f32x8 => (Rotor3x8)
);

#[cfg(feature = "f64")]
impl_slerp_rotor3_wide!(
    f64x2 => (DRotor3x2),
    f64x4 => (DRotor3x4)
);

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
    f32x4 => (Vec2x4, Vec3x4, Vec4x4, Bivec2x4, Bivec3x4, Rotor2x4),
    f32x8 => (Vec2x8, Vec3x8, Vec4x8, Bivec2x8, Bivec3x8, Rotor2x8)
);

#[cfg(feature = "f64")]
impl_slerp_gen!(
    f64 => (DVec2, DVec3, DVec4, DBivec2, DBivec3, DRotor2),
    f64x2 => (DVec2x2, DVec3x2, DVec4x2, DBivec2x2, DBivec3x2, DRotor2x2),
    f64x4 => (DVec2x4, DVec3x4, DVec4x4, DBivec2x4, DBivec3x4, DRotor2x4)
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::EqualsEps;
    use std::f32::consts::*;
    #[test]
    pub fn slerp_in_xy_plane() {
        let rotation = Rotor3::from_rotation_xy(2. * FRAC_PI_3);

        // Expected to be a rotation by angle PI/6
        let interpolated = Rotor3::identity().slerp(rotation, 1. / 4.);

        let rotated = Vec3::unit_x().rotated_by(interpolated);
        let expected = Vec3::new(3f32.sqrt() / 2., 0.5, 0.);
        assert!(rotated.eq_eps(expected))
    }

    #[test]
    pub fn slerp_in_zx_plane() {
        let rotation = Rotor3::from_rotation_zx(2. * FRAC_PI_3);

        // Expected to be a rotation by angle PI/6
        let interpolated = Rotor3::identity().slerp(rotation, 1. / 4.);

        let rotated = Vec3::unit_z().rotated_by(interpolated);
        let expected = Vec3::new(0.5, 0., 3f32.sqrt() / 2.);
        assert!(rotated.eq_eps(expected))
    }
}
