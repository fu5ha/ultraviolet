//! Rotors, i.e. constructs that describe and perform rotations.
//!
//! A rotor is the geometric algebra analog of the Quaternion, and they
//! end up being mathematically equivalent. They are good for doing the same
//! sorts of things, and for the most part you can use rotors just like you
//! would a quaternion, if you're already familiar with using those. However,
//! they are significantly easier to derive yourself and build intuition for,
//! and they generalize to both lower and higher dimensions than just 3, which
//! is the only space for which quaternions are valuable.
//!
//! A rotor can be thought of in multiple ways, the first of which
//! is that a rotor is the result of the 'geometric product' of two vectors,
//! denoted for two vectors `u` and `v` as simply `uv`. This operation is
//! defined as
//!
//! `uv = u · v + u ∧ v`
//!
//! As can be seen, this operation results in the addition of two different
//! types of values: first, the dot product will result in a scalar, and second,
//! the exterior (wedge) product will result in a bivector. The addition of these two different
//! types is not defined, but can be understood in a similar way as complex numbers,
//! i.e. as a 'bundle' of two different kinds of values.
//!
//! The reason we call this type of value a 'rotor' is that if you both left- and
//! right-multiply (using the geometric product) a rotor with a vector, you will
//! rotate the sandwiched vector. For example, if you start with two vectors,
//! `a` and `b`, and create a rotor `ab` from them, then rotate a vector `u` with this
//! rotor by doing `ba u ab`, you will end up rotating the vector `u` by in the plane
//! that corresponds to `a ∧ b` (i.e. the plane which is parallel with both vectors), by
//! twice the angle between `a` and `b`, in the opposite direction of the one that would
//! bring `a` towards `b` within that plane.
//!
//! In `ultraviolet`, the `Mul` trait is implemented for Rotors such that doing
//!
//! `rotor * vec`
//!
//! will rotate the Vector `vec` by the Rotor `rotor`.
//!
//! To compose rotations, simply left-multiply the rotor by another one in the same
//! way that matrix composition works. For example,
//!
//! `rotor_ab = rotor_b * rotor_a`
//!
//! Will result in the composition of `rotor_b` and `rotor_a` such that `rotor_ab` encodes
//! a rotation as though `rotor_a` was applied *and then* `rotor_b` was applied.
//!
//! Note that *composition* of rotors is *more efficient*
//! than composition of matrices, however, the operation of rotating a vector by a rotor, i.e. the
//! `rotor * vec` product,  is *more expensive* to
//! compute than the `matrix * vec` product. So, rotors are excellent for *building* and *interpolating*
//! rotations, but it may be preferable to convert them into matrices before applying them to
//! vectors/points, if the same rotation will be applied to many vectors.

use crate::util::*;
use crate::*;

use std::ops::*;

macro_rules! rotor2s {
    ($($rn:ident => ($mt:ident, $vt:ident, $bt:ident, $t:ident)),+) => {
        $(
        /// A Rotor in 2d space.
        ///
        /// Please see the module level documentation for more information on rotors!
        #[derive(Clone, Copy, Debug, PartialEq)]
        #[repr(C)]
        pub struct $rn {
            pub s: $t,
            pub bv: $bt,
        }

        derive_default_identity!($rn);

        impl $rn {
            #[inline]
            pub const fn new(scalar: $t, bivector: $bt) -> Self {
                Self {
                    s: scalar,
                    bv: bivector,
                }
            }

            #[inline]
            pub fn identity() -> Self {
                Self {
                    s: $t::splat(1.0),
                    bv: $bt::zero(),
                }
            }

            /// Construct a Rotor that rotates one vector to another.
            ///
            /// A rotation between antiparallel vectors is **undefined**!
            #[inline]
            pub fn from_rotation_between(from: $vt, to: $vt) -> Self {
                Self::new(
                    $t::splat(1.0) + to.dot(from),
                    to.wedge(from)).normalized()
            }

            /// Construct a rotor given a bivector which defines a plane and rotation orientation,
            /// and a rotation angle.
            ///
            /// `plane` must be normalized!
            ///
            /// This is the equivalent of an axis-angle rotation.
            #[inline]
            pub fn from_angle_plane(angle: $t, plane: $bt) -> Self {
                let half_angle = angle * $t::splat(0.5);
                let (sin, cos) = half_angle.sin_cos();
                Self::new(cos, plane * -sin)
            }
            /// Construct a rotor given only an angle. This is possible in 2d since there is only one
            /// possible plane of rotation. However, there are two possible orientations. This function
            /// uses the common definition of positive angle in 2d as meaning the direction which brings
            /// the x unit vector towards the y unit vector.
            #[inline]
            pub fn from_angle(angle: $t) -> Self {
                let half_angle = angle / $t::splat(2.0);
                let (sin, cos) = half_angle.sin_cos();
                Self::new(cos, $bt::new(-sin))
            }

            #[inline]
            pub fn mag_sq(&self) -> $t {
                self.s * self.s + self.bv.mag_sq()
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
            #[must_use = "Did you mean to use `.normalize()` to normalize `self` in place?"]
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

            #[inline]
            pub fn dot(&self, rhs: Self) -> $t {
                self.s * rhs.s + self.bv.dot(rhs.bv)
            }

            /// Rotates this rotor by another rotor in-place. Note that if you
            /// are looking to *compose* rotations, you should *NOT* use this
            /// operation and rather just use regular left-multiplication like
            /// for matrix composition.
            #[inline]
            pub fn rotate_by(&mut self, other: Self) {
                let b = *self;
                let a = other;
                let sa2_plus_baxy2 = a.s.mul_add(a.s, a.bv.xy * a.bv.xy);

                self.s = (a.s - b.s) * a.bv.xy * b.bv.xy
                    + b.s * sa2_plus_baxy2;
                self.bv.xy = b.bv.xy * sa2_plus_baxy2;
            }

            /// Rotates this rotor by another rotor and returns the result. Note that if you
            /// are looking to *compose* rotations, you should *NOT* use this
            /// operation and rather just use regular left-multiplication like
            /// for matrix composition.
            #[inline]
            pub fn rotated_by(mut self, other: Self) -> Self {
                self.rotate_by(other);
                self
            }

            /// Rotates a vector by this rotor.
            ///
            /// `self` *must* be normalized!
            #[inline]
            pub fn rotate_vec(self, vec: &mut $vt) {
                let fx = self.s * vec.x + self.bv.xy * vec.y;
                let fy = self.s * vec.y - (self.bv.xy * vec.x);

                vec.x = self.s * fx + self.bv.xy * fy;
                vec.y = self.s * fy - (self.bv.xy * fx);
            }

            #[inline]
            pub fn into_matrix(self) -> $mt {
                let s2_minus_bxy2 = self.s * self.s - self.bv.xy * self.bv.xy;
                let two_s_bxy = $t::splat(2.0) * self.s * self.bv.xy;

                $mt::new(
                    $vt::new(
                        s2_minus_bxy2,
                        -two_s_bxy),
                    $vt::new(
                        two_s_bxy,
                        s2_minus_bxy2))
            }

            #[inline]
            pub fn layout() -> alloc::alloc::Layout {
                alloc::alloc::Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<$t>()).unwrap()
            }
        }

        impl From<$rn> for $mt {
            #[inline]
            fn from(rotor: $rn) -> $mt {
                rotor.into_matrix()
            }
        }

        impl EqualsEps for $rn {
            fn eq_eps(self, other: Self) -> bool {
                self.s.eq_eps(other.s) && self.bv.eq_eps(other.bv)
            }
        }

        /// The composition of `self` with `q`, i.e. `self * q` gives the rotation as though
        /// you first perform `q` and then `self`.
        impl Mul for $rn {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: Self) -> Self {
                Self {
                    s: self.s * rhs.s - (self.bv.xy * rhs.bv.xy),
                    bv: $bt {
                        xy: self.s * rhs.bv.xy + rhs.s * self.bv.xy,
                    }
                }
            }
        }

        impl AddAssign for $rn {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.s += rhs.s;
                self.bv += rhs.bv;
            }
        }

        impl Add for $rn {
            type Output = Self;
            #[inline]
            fn add(mut self, rhs: Self) -> Self {
                self += rhs;
                self
            }
        }

        impl SubAssign for $rn {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.s -= rhs.s;
                self.bv -= rhs.bv;
            }
        }

        impl Sub for $rn {
            type Output = Self;
            #[inline]
            fn sub(mut self, rhs: Self) -> Self {
                self -= rhs;
                self
            }
        }

        impl Mul<$vt> for $rn {
            type Output = $vt;
            #[inline]
            fn mul(self, mut rhs: $vt) -> $vt {
                self.rotate_vec(&mut rhs);
                rhs
            }
        }

        impl MulAssign<$t> for $rn {
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                self.s *= rhs;
                self.bv *= rhs;
            }
        }

        impl Mul<$t> for $rn {
            type Output = Self;
            #[inline]
            fn mul(mut self, rhs: $t) -> Self {
                self *= rhs;
                self
            }
        }

        impl Mul<$rn> for $t {
            type Output = $rn;
            #[inline]
            fn mul(self, rotor: $rn) -> $rn {
                rotor * self
            }
        }

        impl DivAssign<$t> for $rn {
            #[inline]
            fn div_assign(&mut self, rhs: $t) {
                self.s /= rhs;
                self.bv /= rhs;
            }
        }

        impl Div<$t> for $rn {
            type Output = Self;
            #[inline]
            fn div(mut self, rhs: $t) -> Self {
                self /= rhs;
                self
            }
        }
        )+
    }
}

rotor2s!(
    Rotor2 => (Mat2, Vec2, Bivec2, f32),
    Rotor2x4 => (Mat2x4, Vec2x4, Bivec2x4, f32x4),
    Rotor2x8 => (Mat2x8, Vec2x8, Bivec2x8, f32x8)
);

#[cfg(feature = "f64")]
rotor2s!(
    DRotor2 => (DMat2, DVec2, DBivec2, f64),
    DRotor2x2 => (DMat2x2, DVec2x2, DBivec2x2, f64x2),
    DRotor2x4 => (DMat2x4, DVec2x4, DBivec2x4, f64x4)
);

macro_rules! rotor3s {
    ($($rn:ident => ($mt:ident, $vt:ident, $bt:ident, $t:ident)),+) => {
        $(
        /// A Rotor in 3d space.
        ///
        /// Please see the module level documentation for more information on rotors!
        #[derive(Clone, Copy, Debug, PartialEq)]
        #[repr(C)]
        pub struct $rn {
            pub s: $t,
            pub bv: $bt,
        }

        derive_default_identity!($rn);

        impl $rn {
            #[inline]
            pub const fn new(scalar: $t, bivector: $bt) -> Self {
                Self {
                    s: scalar,
                    bv: bivector,
                }
            }

            #[inline]
            pub fn identity() -> Self {
                Self {
                    s: $t::splat(1.0),
                    bv: $bt::zero(),
                }
            }

            /// Construct a Rotor that rotates one vector to another.
            #[inline]
            pub fn from_rotation_between(from: $vt, to: $vt) -> Self {
                Self::new(
                    $t::splat(1.0) + to.dot(from),
                    to.wedge(from)).normalized()
            }

            /// Construct a rotor given a bivector which defines a plane and rotation orientation,
            /// and a rotation angle.
            ///
            /// `plane` must be normalized!
            ///
            /// This is the equivalent of an axis-angle rotation.
            #[inline]
            pub fn from_angle_plane(angle: $t, plane: $bt) -> Self {
                let half_angle = angle * $t::splat(0.5);
                let (sin, cos) = half_angle.sin_cos();
                Self::new(cos, plane * -sin)
            }

            /// Return the angle and the normalized plane of the rotation represented by self.
            /// The value of the returned angle is between 0 and PI.
            #[inline]
            pub fn into_angle_plane(self) -> ($t, $bt) {
                let cos_half_angle = self.s;
                let sin_half_angle = self.bv.mag();
                let half_angle = sin_half_angle.atan2(cos_half_angle);
                (half_angle * 2., -self.bv.normalized())

            }

            /// Multiply the angle of the rotation represented by self by `scale`.
            #[inline]
            pub fn scale_by(&mut self, scale: $t) {
                *self = self.scaled_by(scale)
            }

            /// Return a rotor representing the same rotatation as `self` but with an angle
            /// multiplied by `scale`
            #[inline]
            #[must_use]
            pub fn scaled_by(self, scale: $t) -> Self {
                let (angle, plane) = self.into_angle_plane();
                Self::from_angle_plane(angle * scale, plane)
            }

            /// Create new Rotor from a rotation in the xy plane (also known as
            /// "around the z axis").
            #[inline]
            pub fn from_rotation_xy(angle: $t) -> Self {
                Self::from_angle_plane(angle, $bt::unit_xy())
            }

            /// Create new Rotor from a rotation in the xz plane (also known as
            /// "around the y axis").
            #[inline]
            pub fn from_rotation_xz(angle: $t) -> Self {
                Self::from_angle_plane(angle, $bt::unit_xz())
            }

            /// Create new Rotor from a rotation in the yz plane (also known as
            /// "around the x axis").
            #[inline]
            pub fn from_rotation_yz(angle: $t) -> Self {
                Self::from_angle_plane(angle, $bt::unit_yz())
            }

            /// Angles are applied in the order roll -> pitch -> yaw
            ///
            /// - Roll is rotation inside the xy plane ("around the z axis")
            /// - Pitch is rotation inside the yz plane ("around the x axis")
            /// - Yaw is rotation inside the xz plane ("around the y axis")
            #[inline]
            pub fn from_euler_angles(roll: $t, pitch: $t, yaw: $t) -> Self {
                Self::from_angle_plane(yaw, $bt::unit_xz())
                    * Self::from_angle_plane(pitch, $bt::unit_yz())
                    * Self::from_angle_plane(roll, $bt::unit_xy())
            }

            #[inline]
            pub fn mag_sq(&self) -> $t {
                self.s * self.s + self.bv.mag_sq()
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
            #[must_use = "Did you mean to use `.normalize()` to normalize `self` in place?"]
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

            #[inline]
            pub fn dot(&self, rhs: Self) -> $t {
                self.s * rhs.s + self.bv.dot(rhs.bv)
            }

            /// Rotates this rotor by another rotor in-place. Note that if you
            /// are looking to *compose* rotations which will then be applied to another object/vector
            /// (you probably are), you should
            /// *NOT* use this operation. Rather, just use regular left-multiplication
            /// as in matrix composition, i.e.
            ///
            /// ```rs
            /// second_rotor * first_rotor
            /// ```
            #[inline]
            pub fn rotate_by(&mut self, rhs: Self) {
                // TODO make this faster by adding intermediate factored object
                let b = *self;
                let a = rhs;
                let two = $t::splat(2.0);
                let sa2 = a.s * a.s;
                let baxy2 = a.bv.xy * a.bv.xy;
                let baxz2 = a.bv.xz * a.bv.xz;
                let bayz2 = a.bv.yz * a.bv.yz;
                let sa_baxy = a.s * a.bv.xy;
                let sa_baxz = a.s * a.bv.xz;
                let sa_bayz = a.s * a.bv.yz;
                let baxy_baxz = a.bv.xy * a.bv.xz;
                let baxy_bayz = a.bv.xy * a.bv.yz;
                let baxz_bayz = a.bv.xz * a.bv.yz;
                let two_bbxy = two * b.bv.xy;
                let two_bbxz = two * b.bv.xz;
                let two_bbyz = two * b.bv.yz;

                self.s = (sa2 + baxy2 + baxz2 + bayz2) * b.s;

                self.bv.xy = (sa2 + baxy2 - baxz2 - bayz2) * b.bv.xy
                    + (baxy_baxz + sa_bayz) * two_bbxz
                    + (baxy_bayz - sa_baxz) * two_bbyz;

                self.bv.xz = (sa2 - baxy2 + baxz2 - bayz2) * b.bv.xz
                    + (baxy_baxz - sa_bayz) * two_bbxy
                    + (baxz_bayz + sa_baxy) * two_bbyz;

                self.bv.yz = (sa2 - baxy2 - baxz2 + bayz2) * b.bv.yz
                    + (baxy_bayz + sa_baxz) * two_bbxy
                    + (baxz_bayz - sa_baxy) * two_bbxz;
            }

            /// Rotates this rotor by another rotor and returns the result. Note that if you
            /// are looking to *compose* rotations, you should *NOT* use this
            /// operation and rather just use regular left-multiplication like
            /// as in matrix composition, i.e.
            ///
            /// ```rs
            /// second_rotor * first_rotor
            /// ```
            #[inline]
            pub fn rotated_by(mut self, rhs: Self) -> Self {
                self.rotate_by(rhs);
                self
            }

            /// Rotates a vector by this rotor.
            ///
            /// `self` *must* be normalized!
            #[inline]
            pub fn rotate_vec(self, vec: &mut $vt) {
                // see derivation/rotor3_rotate_vec_derivation for a derivation
                // f = geometric product of (self)(vec)
                let fx = self.s * vec.x + self.bv.xy * vec.y + self.bv.xz * vec.z;
                let fy = self.s * vec.y - self.bv.xy * vec.x + self.bv.yz * vec.z;
                let fz = self.s * vec.z - self.bv.xz * vec.x - self.bv.yz * vec.y;
                let fw = self.bv.xy * vec.z - self.bv.xz * vec.y + self.bv.yz * vec.x;

                // result = geometric product of (f)(self~)
                vec.x = self.s * fx + self.bv.xy * fy + self.bv.xz * fz + self.bv.yz * fw;
                vec.y = self.s * fy - self.bv.xy * fx - self.bv.xz * fw + self.bv.yz * fz;
                vec.z = self.s * fz + self.bv.xy * fw - self.bv.xz * fx - self.bv.yz * fy;
            }

            /// Rotates multiple vectors by this rotor.
            ///
            /// This will be faster than calling `rotate_vec` individually on many vecs
            /// as intermediate values can be precomputed once and applied to each vector.
            ///
            /// `self` must be normalized!
            pub fn rotate_vecs(self, vecs: &mut [$vt]) {
                let s2 = self.s * self.s;
                let bxy2 = self.bv.xy * self.bv.xy;
                let bxz2 = self.bv.xz * self.bv.xz;
                let byz2 = self.bv.yz * self.bv.yz;
                let s_bxy = self.s * self.bv.xy;
                let s_bxz = self.s * self.bv.xz;
                let s_byz = self.s * self.bv.yz;
                let bxz_byz = self.bv.xz * self.bv.yz;
                let bxy_byz = self.bv.xy * self.bv.yz;
                let bxy_bxz = self.bv.xy * self.bv.xz;

                let xa = s2 - bxy2 - bxz2 + byz2;
                let xb = s_bxy - bxz_byz;
                let xc = s_bxz + bxy_byz;

                let ya = -(bxz_byz + s_bxy);
                let yb = s2 - bxy2 + bxz2 - byz2;
                let yc = s_byz - bxy_bxz;

                let za = bxy_byz - s_bxz;
                let zb = bxy_bxz + s_byz;
                let zc = -(s2 + bxy2 - bxz2 - byz2);

                for vec in vecs {
                    let two_vx = vec.x + vec.x;
                    let two_vy = vec.y + vec.y;
                    let two_vz = vec.z + vec.z;

                    vec.x = vec.x * xa + two_vy * xb + two_vz * xc;
                    vec.y = two_vx * ya + vec.y * yb + two_vz * yc;
                    vec.z = two_vx * za - two_vy * zb - vec.z * zc;
                }
            }

            #[inline]
            pub fn into_matrix(self) -> $mt {
                let s2 = self.s * self.s;
                let bxy2 = self.bv.xy * self.bv.xy;
                let bxz2 = self.bv.xz * self.bv.xz;
                let byz2 = self.bv.yz * self.bv.yz;
                let s_bxy = self.s * self.bv.xy;
                let s_bxz = self.s * self.bv.xz;
                let s_byz = self.s * self.bv.yz;
                let bxz_byz = self.bv.xz * self.bv.yz;
                let bxy_byz = self.bv.xy * self.bv.yz;
                let bxy_bxz = self.bv.xy * self.bv.xz;

                let two = $t::splat(2.0);

                $mt::new(
                    $vt::new(
                        s2 - bxy2 - bxz2 + byz2,
                        -two * (bxz_byz + s_bxy),
                        two * (bxy_byz - s_bxz)),
                    $vt::new(
                        two * (s_bxy - bxz_byz),
                        s2 - bxy2 + bxz2 - byz2,
                        -two * (s_byz + bxy_bxz)
                    ),
                    $vt::new(
                        two * (s_bxz + bxy_byz),
                        two * (s_byz - bxy_bxz),
                        s2 + bxy2 - bxz2 - byz2
                    )
                )
            }

            /// Convert this rotor into an array that represents a quaternion. This is in the form
            /// `[vector, scalar]`.
            #[inline]
            pub fn into_quaternion_array(self) -> [$t; 4] {
                [-self.bv.yz, self.bv.xz, -self.bv.xy, self.s]
            }

            /// Convert an array that represents a quaternion in the form `[vector, scalar]` into a
            /// rotor.
            #[inline]
            pub fn from_quaternion_array(array: [$t; 4]) -> Self {
                Self::new(array[3], $bt::new(-array[2], array[1], -array[0]))
            }

            #[inline]
            pub fn layout() -> alloc::alloc::Layout {
                alloc::alloc::Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<$t>()).unwrap()
            }
        }

        impl From<$rn> for $mt {
            #[inline]
            fn from(rotor: $rn) -> $mt {
                rotor.into_matrix()
            }
        }

        impl EqualsEps for $rn {
            #[inline]
            fn eq_eps(self, other: Self) -> bool {
                self.s.eq_eps(other.s) && self.bv.eq_eps(other.bv)
            }
        }

        /// The composition of `self` with `q`, i.e. `self * q` gives the rotation as though
        /// you first perform `q` and then `self`.
        impl Mul for $rn {
            type Output = Self;

            /// The composition of `self` with `q`, i.e. `self * q` gives the rotation as though
            /// you first perform `q` and then `self`.
            #[inline]
            fn mul(self, q: Self) -> Self {
                Self {
                    s: self.s * q.s - self.bv.xy * q.bv.xy - self.bv.xz * q.bv.xz - self.bv.yz * q.bv.yz,
                    bv: $bt {
                        xy: self.bv.xy * q.s + self.s * q.bv.xy + self.bv.yz * q.bv.xz - self.bv.xz * q.bv.yz,
                        xz: self.bv.xz * q.s + self.s * q.bv.xz - self.bv.yz * q.bv.xy + self.bv.xy * q.bv.yz,
                        yz: self.bv.yz * q.s + self.s * q.bv.yz + self.bv.xz * q.bv.xy - self.bv.xy * q.bv.xz,
                    }
                }
            }
        }

        impl AddAssign for $rn {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.s += rhs.s;
                self.bv += rhs.bv;
            }
        }

        impl Add for $rn {
            type Output = Self;
            #[inline]
            fn add(mut self, rhs: Self) -> Self {
                self += rhs;
                self
            }
        }

        impl SubAssign for $rn {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.s -= rhs.s;
                self.bv -= rhs.bv;
            }
        }

        impl Sub for $rn {
            type Output = Self;
            #[inline]
            fn sub(mut self, rhs: Self) -> Self {
                self -= rhs;
                self
            }
        }

        impl Mul<$vt> for $rn {
            type Output = $vt;
            #[inline]
            fn mul(self, mut rhs: $vt) -> $vt {
                self.rotate_vec(&mut rhs);
                rhs
            }
        }

        impl MulAssign<$t> for $rn {
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                self.s *= rhs;
                self.bv *= rhs;
            }
        }

        impl Mul<$t> for $rn {
            type Output = Self;
            #[inline]
            fn mul(mut self, rhs: $t) -> Self {
                self *= rhs;
                self
            }
        }

        impl Mul<$rn> for $t {
            type Output = $rn;
            #[inline]
            fn mul(self, rotor: $rn) -> $rn {
                rotor * self
            }
        }

        impl DivAssign<$t> for $rn {
            #[inline]
            fn div_assign(&mut self, rhs: $t) {
                self.s /= rhs;
                self.bv /= rhs;
            }
        }

        impl Div<$t> for $rn {
            type Output = Self;
            #[inline]
            fn div(mut self, rhs: $t) -> Self {
                self /= rhs;
                self
            }
        }
        )+
    }
}

rotor3s!(
    Rotor3 => (Mat3, Vec3, Bivec3, f32),
    Rotor3x4 => (Mat3x4, Vec3x4, Bivec3x4, f32x4),
    Rotor3x8 => (Mat3x8, Vec3x8, Bivec3x8, f32x8)
);

#[cfg(feature = "f64")]
rotor3s!(
    DRotor3 => (DMat3, DVec3, DBivec3, f64),
    DRotor3x2 => (DMat3x2, DVec3x2, DBivec3x2, f64x2),
    DRotor3x4 => (DMat3x4, DVec3x4, DBivec3x4, f64x4)
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn rotate_vector_roundtrip() {
        let a = Vec3::new(1.0, 2.0, -5.0).normalized();
        let b = Vec3::new(1.0, 1.0, 1.0).normalized();
        let c = Vec3::new(2.0, 3.0, -3.0).normalized();
        let rotor_ab = Rotor3::from_rotation_between(a, b);
        let rotor_bc = Rotor3::from_rotation_between(b, c);
        let rot_ab = rotor_ab * a;
        let rot_bc = rotor_bc * b;
        let rot_abc = rotor_bc * (rotor_ab * a);
        println!("{:?} = {:?}", rot_ab, b);
        println!("{:?} = {:?}", rot_bc, c);
        println!("{:?} = {:?}", rot_abc, c);
        assert!(rot_ab.eq_eps(b));
        assert!(rot_bc.eq_eps(c));
        assert!(rot_abc.eq_eps(c));
    }

    #[test]
    pub fn rotate_rotor_trivial() {
        let a = Vec3::new(1.0, 2.0, -5.0).normalized();
        let b = Vec3::new(1.0, 1.0, 1.0).normalized();
        let c = Vec3::new(2.0, 3.0, -3.0).normalized();
        let r_ab = Rotor3::from_rotation_between(a, b);
        let r_bc = Rotor3::from_rotation_between(b, c);
        let res = r_ab.rotated_by(r_bc).rotated_by(r_bc.reversed());
        println!("{:?} {:?}", r_ab, res);
        assert!(r_ab.eq_eps(res));
    }

    #[test]
    pub fn compose_rotor_roundtrip() {
        let a = Vec3::new(0.25, -5.0, 1.0).normalized();
        let b = Vec3::new(-5.0, 2.0, 4.0).normalized();
        let c = Vec3::new(-3.0, 0.0, -1.0).normalized();
        let rotor_ab = Rotor3::from_rotation_between(a, b);
        let rotor_bc = Rotor3::from_rotation_between(b, c);
        let rotor_abbc = rotor_bc * rotor_ab;
        let res = rotor_abbc * a;
        println!("{:#?} {:#?}", rotor_abbc, res);
        assert!(c.eq_eps(res));
    }

    #[test]
    pub fn rotor_interp_trivial() {
        let i = Rotor3::identity();

        let interp = i.lerp(i, 0.5);

        println!("{:#?} ::: {:#?}", i, interp);
        assert!(interp.eq_eps(i))
    }

    #[test]
    #[allow(clippy::eq_op)]
    pub fn rotor_equality() {
        let i = Rotor3::identity();
        assert_eq!(i, i);
    }

    #[test]
    pub fn angle_plane_roundtrip() {
        let angle = 0.32;
        let plane = Bivec3::new(0.2, 0.4, 0.7).normalized();
        let rotor = Rotor3::from_angle_plane(angle, plane);
        let (angle_, plane_) = rotor.into_angle_plane();
        assert!(Rotor3::from_angle_plane(angle_, plane_).eq_eps(rotor));
        let angle = -0.32;
        let plane = Bivec3::new(0.2, 0.4, 0.7).normalized();
        let rotor = Rotor3::from_angle_plane(angle, plane);
        let (angle_, plane_) = rotor.into_angle_plane();
        assert!(Rotor3::from_angle_plane(angle_, plane_).eq_eps(rotor));
    }

    #[test]
    pub fn quaternion_convertion_roundtrip() {
        let a = Vec3::new(1.0, 2.0, -5.0).normalized();
        let b = Vec3::new(1.0, 1.0, 1.0).normalized();
        let rotor = Rotor3::from_rotation_between(a, b);
        assert_eq!(
            rotor,
            Rotor3::from_quaternion_array(rotor.into_quaternion_array())
        );
    }

    #[test]
    pub fn rotor_scaling() {
        use std::f32::consts::PI;

        let axis = Vec3::new(0.42, 0.123, 0.789).normalized(); //aribitrary rotation axis
        let plane = Bivec3::from_normalized_axis(axis).normalized();
        let angle = PI / 10.;

        // rotation of angle pi/10 on the axis;
        let rotation_1 = Rotor3::from_angle_plane(angle, plane);

        let fraction = 1.234;

        let scaled_rotor_1 = Rotor3::from_angle_plane(fraction * angle, plane);
        let scaled_rotor_2 = rotation_1.scaled_by(fraction);
        assert!(scaled_rotor_1.eq_eps(scaled_rotor_2));
    }

    // This test exists because Rotor3 used to implement PartialEq without DRotor3 getting the same
    // impl. Use `cargo test --all-features` to run
    #[cfg(feature = "f64")]
    #[test]
    #[allow(clippy::eq_op)]
    pub fn drotor_equality() {
        let i = DRotor3::identity();
        assert_eq!(i, i);
    }
}
