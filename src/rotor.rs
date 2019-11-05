//! A rotor can be thought of in multiple ways, the first of which
//! is that it is the result of the 'geometric product' of two vectors,
//! denoted for two vectors `u` and `v` as simply `uv`. This operation is
//! defined as
//!
//! `uv = u · v + u ∧ v`
//!
//! As can be seen, this operation results in the addition of two different
//! types of values: first, the dot product will result in a scalar, and second,
//! the outer product will result in a bivector. The addition of these two different
//! types is not defined, but can be understood in a similar way as complex numbers,
//! i.e. as a 'bundle' of two different kinds of values.
//!
//! The reason we call this type of value a 'rotor' is that if you both left- and
//! right-multiply (using the geometric product) a rotor with a vector, you will
//! rotate the sandwiched vector. For example, if you start with two vectors,
//! `a` and `b`, and create a rotor `ab` from them, then rotate a vector `u` with this
//! rotor by doing `ba u ab`, you will end up rotating the vector `u` by in the plane
//! that corresponds to `a ∧ b` (i.e. the plane which is parallel with both vectors), by
//! twice the angle between `a` and `b`.
//!
//! In `ultraviolet`, the `Mul` trait is implemented for Rotors such that doing
//!
//! `rotor * vec`
//!
//! will rotate the Vector `vec` by the Rotor `rotor`.

use crate::bivec::*;
use crate::util::*;
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

            /// Construct a Rotor that rotates one vector to another.
            #[inline]
            pub fn rotation_between(from: $vt, to: $vt) -> Self {
                Self::new(
                    $t::from(1.0) + to.dot(from),
                    to.wedge(from)).normalized()
            }

            /// Construct a vector given an angle and a bivector which defines a plane and rotation
            /// orientation.
            ///
            /// This is the equivalent of an axis-angle rotation. The plane bivector
            /// must be normalized.
            #[inline]
            pub fn angle_plane(angle: $t, mut plane: $bt) -> Self {
                let two = $t::from(2.0);
                let sina = (angle / two).sin();
                plane *= -sina;
                Self::new((angle / two).cos(), plane)
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

            /// Rotates this rotor by another rotor in-place. `self` *must* be normalized!
            #[inline]
            pub fn rotate_by(&mut self, other: Self) {
                let b = *self;
                let a = other;
                let sa2_plus_baxy2 = a.s * a.s + a.bv.xy * a.bv.xy;

                self.s = (a.s - b.s) * a.bv.xy * b.bv.xy
                    + b.s * sa2_plus_baxy2;
                self.bv.xy = b.bv.xy * sa2_plus_baxy2;
            }

            /// Rotates this rotor by another rotor and returns the result. `self` *must* be normalized!
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
                let bxy2 = self.bv.xy * self.bv.xy;
                let two = $t::from(2.0);

                let v = *vec;

                vec.x = self.s * (self.s * v.x + two * self.bv.xy * v.y) - bxy2 * v.x;
                vec.y = self.s * (self.s * v.y - two * self.bv.xy * v.x) - bxy2 * v.y;
            }
        }

        impl EqualsEps for $rn {
            fn eq_eps(self, other: Self) -> bool {
                self.s.eq_eps(other.s) && self.bv.eq_eps(other.bv)
            }
        }

        impl Mul for $rn {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: Self) -> Self {
                Self {
                    s: self.s * rhs.s - self.bv.xy * rhs.bv.xy,
                    bv: $bt {
                        xy: self.s * rhs.bv.xy + rhs.s * self.bv.xy
                    }
                }
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

            /// Construct a Rotor that rotates one vector to another.
            #[inline]
            pub fn rotation_between(from: $vt, to: $vt) -> Self {
                Self::new(
                    $t::from(1.0) + to.dot(from),
                    to.wedge(from)).normalized()
            }

            /// Construct a vector given an angle and a bivector which defines a plane and rotation
            /// orientation.
            ///
            /// This is the equivalent of an axis-angle rotation. The plane bivector
            /// must be normalizes.
            #[inline]
            pub fn angle_plane(angle: $t, mut plane: $bt) -> Self {
                let two = $t::from(2.0);
                let sina = (angle / two).sin();
                plane *= -sina;
                Self::new((angle / two).cos(), plane)
            }

            #[inline]
            pub fn mag_sq(&self) -> $t {
                self.s * self.s
                    + self.bv.xy * self.bv.xy
                    + self.bv.xz * self.bv.xz
                    + self.bv.yz * self.bv.yz
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

            /// Rotates this rotor by another rotor in-place. `self` *must* be normalized!
            #[inline]
            pub fn rotate_by(&mut self, rhs: Self) {
                let b = *self;
                let a = rhs;
                let two = $t::from(2.0);
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

            /// Rotates this rotor by another rotor and returns the result. `self` *must* be normalized!
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
                let s2 = self.s * self.s;
                let bxy2 = self.bv.xy * self.bv.xy;
                let bxz2 = self.bv.xz * self.bv.xz;
                let byz2 = self.bv.yz * self.bv.yz;
                let two = $t::from(2.0);
                let s_bxy = self.s * self.bv.xy;
                let s_bxz = self.s * self.bv.xz;
                let s_byz = self.s * self.bv.yz;
                let bxz_byz = self.bv.xz * self.bv.yz;
                let bxy_byz = self.bv.xy * self.bv.yz;
                let bxy_bxz = self.bv.xy * self.bv.xz;
                let two_vx = two * vec.x;
                let two_vy = two * vec.y;
                let two_vz = two * vec.z;

                vec.x = vec.x  * (s2 - bxy2 - bxz2 + byz2)
                      + two_vy * (s_bxy - bxz_byz)
                      + two_vz * (s_bxz + bxy_byz);
                vec.y = two_vx * -(bxz_byz + s_bxy)
                      + vec.y  * (s2 - bxy2 + bxz2 - byz2)
                      + two_vz * (s_byz - bxy_bxz);
                vec.z = two_vx * (bxy_byz - s_bxz)
                      - two_vy * (bxy_bxz + s_byz)
                      + vec.z  * (s2 + bxy2 - bxz2 - byz2);
            }
        }

        impl EqualsEps for $rn {
            fn eq_eps(self, other: Self) -> bool {
                self.s.eq_eps(other.s) && self.bv.eq_eps(other.bv)
            }
        }

        impl Mul for $rn {
            type Output = Self;
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


        impl Mul<$vt> for $rn {
            type Output = $vt;
            #[inline]
            fn mul(self, mut rhs: $vt) -> $vt {
                self.rotate_vec(&mut rhs);
                rhs
            }
        }
        )+
    }
}

rotor3s!(Rotor3 => (Vec3, Bivec3, f32), WRotor3 => (Wec3, WBivec3, f32x4));

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn rotate_vector_roundtrip() {
        let a = Vec3::new(1.0, 2.0, -5.0).normalized();
        let b = Vec3::new(1.0, 1.0, 1.0).normalized();
        let c = Vec3::new(2.0, 3.0, -3.0).normalized();
        let rotor_ab = Rotor3::rotation_between(a, b);
        let rotor_bc = Rotor3::rotation_between(b, c);
        let rot_ab = rotor_ab * a;
        let rot_bc = rotor_bc * rot_ab;
        let rot_abc = rotor_bc * (rotor_ab * a);
        assert!(rot_ab.eq_eps(b));
        assert!(rot_bc.eq_eps(c));
        assert!(rot_abc.eq_eps(c));
    }

    #[test]
    pub fn rotate_rotor_trivial() {
        let a = Vec3::new(1.0, 2.0, -5.0).normalized();
        let b = Vec3::new(1.0, 1.0, 1.0).normalized();
        let c = Vec3::new(2.0, 3.0, -3.0).normalized();
        let r_ab = Rotor3::rotation_between(a, b);
        let r_bc = Rotor3::rotation_between(b, c);
        let res = r_ab.rotated_by(r_bc).rotated_by(r_bc.reversed());
        println!("{:?} {:?}", r_ab, res);
        assert!(r_ab.eq_eps(res));
    }

    #[test]
    pub fn rotate_rotor_roundtrip() {
        let a = Vec3::new(1.0, 0.0, 0.0).normalized();
        let b = Vec3::new(1.0, 1.0, 0.0).normalized();
        let c = Vec3::new(1.0, 1.0, 1.0).normalized();
        let d = Vec3::new(0.0, 1.0, 0.0).normalized();
        let rotor_ab = Rotor3::rotation_between(a, b);
        let rotor_bc = Rotor3::rotation_between(c, d);
        let rotor_abbc = rotor_bc * rotor_ab;
        let rot = rotor_abbc * Vec3::new(1.0, 0.0, 0.0);
        println!("{:?} {:?} {:?}", rotor_ab, rotor_abbc, rot);
        assert!(rot.eq_eps(Vec3::new(0.0, 1.0, 0.0).normalized()));
    }
}
