//! Vectors and points, i.e. directed line segments and locations.
use crate::bivec::*;
use crate::rotor::*;
use crate::util::*;
use std::ops::*;

use wide::f32x4;

macro_rules! vec2s {
    ($(($n:ident, $bn:ident, $rn:ident, $v3t:ident) => $t:ident),+) => {
        $(
        /// A set of two coordinates which may be interpreted as a vector or point in 2d space.
        ///
        /// Generally this distinction between a point and vector is more of a pain than it is worth
        /// to distinguish on a type level, however when converting to and from homogeneous
        /// coordinates it is quite important.
        #[derive(Clone, Copy, Debug)]
        pub struct $n {
            pub x: $t,
            pub y: $t,
        }

        impl $n {
            #[inline]
            pub fn new(x: $t, y: $t) -> Self {
                $n { x, y }
            }

            #[inline]
            pub fn broadcast(val: $t) -> Self {
                Self::new(val, val)
            }

            #[inline]
            pub fn unit_x() -> Self {
                $n{ x: $t::from(1.0), y: $t::from(0.0) }
            }

            #[inline]
            pub fn unit_y() -> Self {
                $n{ x: $t::from(0.0), y: $t::from(1.0) }
            }

            /// Create a homogeneous 2d *point* from this vector interpreted as a point,
            /// meaning the homogeneous component will start with a value of 1.0.
            #[inline]
            pub fn into_homogeneous_point(self) -> $v3t {
                $v3t { x: self.x, y: self.y, z: $t::from(1.0) }
            }

            /// Create a homogeneous 2d *vector* from this vector,
            /// meaning the homogeneous component will always have a value of 0.0.
            #[inline]
            pub fn into_homogeneous_vector(self) -> $v3t {
                $v3t { x: self.x, y: self.y, z: $t::from(0.0) }
            }

            /// Create a 2d point from a homogeneous 2d *point*, performing
            /// division by the homogeneous component. This should not be used
            /// for homogeneous 2d *vectors*, which will have 0 as their
            /// homogeneous component.
            #[inline]
            pub fn from_homogeneous_point(v: $v3t) -> Self {
                Self { x: v.x / v.z, y: v.y / v.z }
            }

            /// Create a 2d vector from homogeneous 2d *vector*, which simply
            /// discards the homogeneous component.
            #[inline]
            pub fn from_homogeneous_vector(v: $v3t) -> Self {
                v.into()
            }

            #[inline]
            pub fn dot(&self, other: $n) -> $t {
                self.x * other.x + self.y * other.y
            }

            /// The wedge (aka exterior) product of two vectors.
            ///
            /// This operation results in a bivector, which represents
            /// the plane parallel to the two vectors, and which has a
            /// 'oriented area' equal to the parallelogram created by extending
            /// the two vectors, oriented such that the positive direction is the
            /// one which would move `self` closer to `other`.
            #[inline]
            pub fn wedge(&self, other: $n) -> $bn {
                $bn::new(self.x * other.y - other.x * self.y)
            }

            /// The geometric product of this and another vector, which
            /// is defined as the sum of the dot product and the wedge product.
            ///
            /// This operation results in a 'rotor', named as such as it may define
            /// a rotation. The rotor which results from the geometric product
            /// will rotate in the plane parallel to the two vectors, by twice the angle between
            /// them and in the opposite direction (i.e. it will rotate in the direction that would
            /// bring `other` towards `self`, and rotate in that direction by twice the angle between them).
            #[inline]
            pub fn geom(&self, other: $n) -> $rn {
                $rn::new(self.dot(other), self.wedge(other))
            }

            #[inline]
            pub fn rotate_by(&mut self, rotor: $rn) {
                rotor.rotate_vec(self);
            }

            #[inline]
            pub fn rotated_by(mut self, rotor: $rn) -> Self {
                rotor.rotate_vec(&mut self);
                self
            }

            #[inline]
            pub fn reflected(&self, normal: $n) -> Self {
                *self - ($t::from(2.0) * self.dot(normal) * normal)
            }


            #[inline]
            pub fn mag_sq(&self) -> $t {
                self.x * self.x + self.y * self.y
            }

            #[inline]
            pub fn mag(&self) -> $t {
                self.mag_sq().sqrt()
            }

            #[inline]
            pub fn normalize(&mut self) {
                let mag = self.mag();
                self.x /= mag;
                self.y /= mag;
            }

            #[inline]
            pub fn normalized(&self) -> Self {
                let mut r = self.clone();
                r.normalize();
                r
            }

            #[inline]
            pub fn mul_add(&self, mul: $n, add: $n) -> Self {
                $n::new(
                    self.x.mul_add(mul.x, add.x),
                    self.y.mul_add(mul.x, add.x),
                )
            }

            #[inline]
            pub fn abs(&self) -> Self {
                Self::new(self.x.abs(), self.y.abs())
            }

            #[inline]
            pub fn clamp(&mut self, min: Self, max: Self) {
                self.x = self.x.max(min.x).min(max.x);
                self.y = self.y.max(min.y).min(max.y);
            }

            #[inline]
            pub fn clamped(mut self, min: Self, max: Self) -> Self {
                self.clamp(min, max);
                self
            }

            #[inline]
            pub fn map<F>(&self, f: F) -> Self
                where F: Fn($t) -> $t
            {
                $n::new(
                    f(self.x),
                    f(self.y),
                )
            }

            #[inline]
            pub fn apply<F>(&mut self, f: F)
                where F: Fn($t) -> $t
            {
                self.x = f(self.x);
                self.y = f(self.y);
            }

            #[inline]
            pub fn max_by_component(mut self, other: Self) -> Self {
                self.x = self.x.max(other.x);
                self.y = self.y.max(other.y);
                self
            }

            #[inline]
            pub fn min_by_component(mut self, other: Self) -> Self {
                self.x = self.x.min(other.x);
                self.y = self.y.min(other.y);
                self
            }

            #[inline]
            pub fn component_max(&self) -> $t {
                self.x.max(self.y)
            }

            #[inline]
            pub fn component_min(&self) -> $t {
                self.x.min(self.y)
            }

            #[inline]
            pub fn zero() -> Self {
                Self::broadcast($t::from(0.0))
            }

            #[inline]
            pub fn one() -> Self {
                Self::broadcast($t::from(1.0))
            }
        }

        impl EqualsEps for $n {
            fn eq_eps(self, other: Self) -> bool {
                self.x.eq_eps(other.x) && self.y.eq_eps(other.y)
            }
        }

        impl Add for $n {
            type Output = Self;
            #[inline]
            fn add(self, rhs: $n) -> Self {
                $n::new(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl AddAssign for $n {
            #[inline]
            fn add_assign(&mut self, rhs: $n) {
                self.x += rhs.x;
                self.y += rhs.y;
            }
        }

        impl Sub for $n {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: $n) -> Self {
                $n::new(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl SubAssign for $n {
            #[inline]
            fn sub_assign(&mut self, rhs: $n) {
                self.x -= rhs.x;
                self.y -= rhs.y;
            }
        }

        impl Mul for $n {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: $n) -> Self {
                $n::new(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<$n> for $t {
            type Output = $n;
            #[inline]
            fn mul(self, rhs: $n) -> $n {
                $n::new(self * rhs.x, self * rhs.y)
            }
        }

        impl Mul<$t> for $n {
            type Output = $n;
            #[inline]
            fn mul(self, rhs: $t) -> $n {
                $n::new(self.x * rhs, self.y * rhs)
            }
        }

        impl MulAssign for $n {
            #[inline]
            fn mul_assign(&mut self, rhs: $n) {
                self.x *= rhs.x;
                self.y *= rhs.y;
            }
        }

        impl MulAssign<$t> for $n {
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                self.x *= rhs;
                self.y *= rhs;
            }
        }

        impl Div for $n {
            type Output = Self;
            #[inline]
            fn div(self, rhs: $n) -> Self {
                $n::new(self.x / rhs.x, self.y / rhs.y)
            }
        }

        impl Div<$t> for $n {
            type Output = $n;
            #[inline]
            fn div(self, rhs: $t) -> $n {
                $n::new(self.x / rhs, self.y / rhs)
            }
        }

        impl DivAssign for $n {
            #[inline]
            fn div_assign(&mut self, rhs: $n) {
                self.x /= rhs.x;
                self.y /= rhs.y;
            }
        }

        impl DivAssign<$t> for $n {
            #[inline]
            fn div_assign(&mut self, rhs: $t) {
                self.x /= rhs;
                self.y /= rhs;
            }
        }

        impl Neg for $n {
            type Output = $n;
            #[inline]
            fn neg(self) -> $n {
                self * $t::from(-1.0)
            }
        })+
    };
}

vec2s!((Vec2, Bivec2, Rotor2, Vec3) => f32, (Wec2, WBivec2, WRotor2, Wec3) => f32x4);

impl From<[Vec2; 4]> for Wec2 {
    #[inline]
    fn from(vecs: [Vec2; 4]) -> Self {
        Self {
            x: f32x4::new(vecs[0].x, vecs[1].x, vecs[2].x, vecs[3].x),
            y: f32x4::new(vecs[0].y, vecs[1].y, vecs[2].y, vecs[3].y),
        }
    }
}

impl From<Vec3> for Vec2 {
    #[inline]
    fn from(vec: Vec3) -> Self {
        Self { x: vec.x, y: vec.y }
    }
}

impl From<Wec3> for Wec2 {
    #[inline]
    fn from(vec: Wec3) -> Self {
        Self { x: vec.x, y: vec.y }
    }
}

impl Vec2 {
    #[inline]
    pub fn refracted(&mut self, normal: Self, eta: f32) -> Self {
        let n = normal;
        let i = *self;
        let ndi = n.dot(i);
        let k = 1.0 - eta * eta * (1.0 - ndi * ndi);
        if k < 0.0 {
            Self::zero()
        } else {
            i * eta - n * (eta * ndi * k.sqrt())
        }
    }
}

impl Wec2 {
    #[inline]
    pub fn new_splat(x: f32, y: f32) -> Self {
        Self {
            x: f32x4::from(x),
            y: f32x4::from(y),
        }
    }

    #[inline]
    pub fn splat(vec: Vec2) -> Self {
        Self::from([vec, vec, vec, vec])
    }

    #[inline]
    pub fn merge(mask: f32x4, a: Self, b: Self) -> Self {
        Self {
            x: f32x4::merge(mask, a.x, b.x),
            y: f32x4::merge(mask, a.y, b.y),
        }
    }

    #[inline]
    pub fn refracted(&mut self, normal: Self, eta: f32x4) -> Self {
        let n = normal;
        let i = *self;
        let one = f32x4::from(1.0);
        let ndi = n.dot(i);

        let k = one - eta * eta * (one - ndi * ndi);
        let mask = k.cmp_lt(f32x4::from(0.0));

        let out = i * eta - n * (eta * ndi * k.sqrt());

        Self::merge(mask, out, Self::zero())
    }
}

macro_rules! vec3s {
    ($(($n:ident, $bn:ident, $rn:ident, $v4t:ident) => $t:ident),+) => {
        /// A set of three coordinates which may be interpreted as a point or vector in 3d space,
        /// or as a homogeneous 2d vector or point.
        ///
        /// Generally this distinction between a point and vector is more of a pain than it is worth
        /// to distinguish on a type level, however when converting to and from homogeneous
        /// coordinates it is quite important.
        $(#[derive(Clone, Copy, Debug)]
        pub struct $n {
            pub x: $t,
            pub y: $t,
            pub z: $t,
        }

        impl $n {
            #[inline]
            pub fn new(x: $t, y: $t, z: $t) -> Self {
                $n { x, y, z }
            }

            #[inline]
            pub fn broadcast(val: $t) -> Self {
                Self::new(val, val, val)
            }

            #[inline]
            pub fn unit_x() -> Self {
                $n{ x: $t::from(1.0), y: $t::from(0.0), z: $t::from(0.0) }
            }

            #[inline]
            pub fn unit_y() -> Self {
                $n{ x: $t::from(0.0), y: $t::from(1.0), z: $t::from(0.0) }
            }

            #[inline]
            pub fn unit_z() -> Self {
                $n{ x: $t::from(0.0), y: $t::from(0.0), z: $t::from(1.0) }
            }

            /// Create a homogeneous 3d *point* from this vector interpreted as a point,
            /// meaning the homogeneous component will start with a value of 1.0.
            #[inline]
            pub fn into_homogeneous_point(self) -> $v4t {
                $v4t { x: self.x, y: self.y, z: self.z, w: $t::from(1.0) }
            }

            /// Create a homogeneous 3d *vector* from this vector,
            /// meaning the homogeneous component will always have a value of 0.0.
            #[inline]
            pub fn into_homogeneous_vector(self) -> $v4t {
                $v4t { x: self.x, y: self.y, z: self.z, w: $t::from(0.0) }
            }

            /// Create a 3d point from a homogeneous 3d *point*, performing
            /// division by the homogeneous component. This should not be used
            /// for homogeneous 3d *vectors*, which will have 0 as their
            /// homogeneous component.
            #[inline]
            pub fn from_homogeneous_point(v: $v4t) -> Self {
                Self { x: v.x / v.w, y: v.y / v.w, z: v.z / v.w }
            }

            /// Create a 3d vector from homogeneous 2d *vector*, which simply
            /// discards the homogeneous component.
            #[inline]
            pub fn from_homogeneous_vector(v: $v4t) -> Self {
                v.into()
            }


            #[inline]
            pub fn dot(&self, other: $n) -> $t {
                self.x * other.x + self.y * other.y + self.z * other.z
            }

            /// The wedge (aka exterior) product of two vectors.
            ///
            /// This operation results in a bivector, which represents
            /// the plane parallel to the two vectors, and which has a
            /// 'oriented area' equal to the parallelogram created by extending
            /// the two vectors, oriented such that the positive direction is the
            /// one which would move `self` closer to `other`.
            #[inline]
            pub fn wedge(&self, other: $n) -> $bn {
                $bn::new(
                    self.x * other.y - self.y * other.x,
                    self.x * other.z - self.z * other.x,
                    self.y * other.z - self.z * other.y
                )
            }

            /// The geometric product of this and another vector, which
            /// is defined as the sum of the dot product and the wedge product.
            ///
            /// This operation results in a 'rotor', named as such as it may define
            /// a rotation. The rotor which results from the geometric product
            /// will rotate in the plane parallel to the two vectors, by twice the angle between
            /// them and in the opposite direction (i.e. it will rotate in the direction that would
            /// bring `other` towards `self`, and rotate in that direction by twice the angle between them).
            #[inline]
            pub fn geom(&self, other: $n) -> $rn {
                $rn::new(self.dot(other), self.wedge(other))
            }

            #[inline]
            pub fn rotate_by(&mut self, rotor: $rn) {
                rotor.rotate_vec(self);
            }

            #[inline]
            pub fn rotated_by(mut self, rotor: $rn) -> Self {
                rotor.rotate_vec(&mut self);
                self
            }

            #[inline]
            pub fn cross(&self, other: $n) -> Self {
                $n::new(
                    self.y * other.z - self.z * other.y,
                    self.z * other.x - self.x * other.z,
                    self.x * other.y - self.y * other.x,
                )
            }

            #[inline]
            pub fn reflect(&mut self, normal: $n) {
                *self -= $t::from(2.0) * self.dot(normal) * normal;
            }

            #[inline]
            pub fn reflected(&self, normal: $n) -> Self {
                let mut a = *self;
                a.reflect(normal);
                a
            }

            #[inline]
            pub fn mag_sq(&self) -> $t {
                self.x * self.x + self.y * self.y + self.z * self.z
            }

            #[inline]
            pub fn mag(&self) -> $t {
                self.mag_sq().sqrt()
            }

            #[inline]
            pub fn normalize(&mut self) {
                let mag = self.mag();
                self.x /= mag;
                self.y /= mag;
                self.z /= mag;
            }

            #[inline]
            pub fn normalized(&self) -> Self {
                let mut r = self.clone();
                r.normalize();
                r
            }

            #[inline]
            pub fn mul_add(&self, mul: $n, add: $n) -> Self {
                $n::new(
                    self.x.mul_add(mul.x, add.x),
                    self.y.mul_add(mul.y, add.y),
                    self.z.mul_add(mul.z, add.z),
                )
            }

            #[inline]
            pub fn abs(&self) -> Self {
                Self::new(self.x.abs(), self.y.abs(), self.z.abs())
            }

            #[inline]
            pub fn clamp(&mut self, min: Self, max: Self) {
                self.x = self.x.max(min.x).min(max.x);
                self.y = self.y.max(min.y).min(max.y);
                self.z = self.z.max(min.z).min(max.z);
            }

            #[inline]
            pub fn clamped(mut self, min: Self, max: Self) -> Self {
                self.clamp(min, max);
                self
            }

            #[inline]
            pub fn map<F>(&self, f: F) -> Self
                where F: Fn($t) -> $t
            {
                $n::new(
                    f(self.x),
                    f(self.y),
                    f(self.z)
                )
            }

            #[inline]
            pub fn apply<F>(&mut self, f: F)
                where F: Fn($t) -> $t
            {
                self.x = f(self.x);
                self.y = f(self.y);
                self.z = f(self.z);
            }

            #[inline]
            pub fn max_by_component(mut self, other: Self) -> Self {
                self.x = self.x.max(other.x);
                self.y = self.y.max(other.y);
                self.z = self.z.max(other.z);
                self
            }

            #[inline]
            pub fn min_by_component(mut self, other: Self) -> Self {
                self.x = self.x.min(other.x);
                self.y = self.y.min(other.y);
                self.z = self.z.min(other.z);
                self
            }

            #[inline]
            pub fn component_max(&self) -> $t {
                self.x.max(self.y).max(self.z)
            }

            #[inline]
            pub fn component_min(&self) -> $t {
                self.x.min(self.y).min(self.z)
            }

            #[inline]
            pub fn zero() -> Self {
                Self::broadcast($t::from(0.0))
            }

            #[inline]
            pub fn one() -> Self {
                Self::broadcast($t::from(1.0))
            }
        }

        impl EqualsEps for $n {
            fn eq_eps(self, other: Self) -> bool {
                self.x.eq_eps(other.x) && self.y.eq_eps(other.y) && self.z.eq_eps(other.z)
            }
        }

        impl From<[$t; 3]> for $n {
            #[inline]
            fn from(comps: [$t; 3]) -> Self {
                Self::new(comps[0], comps[1], comps[2])
            }
        }

        impl Add for $n {
            type Output = Self;
            #[inline]
            fn add(self, rhs: $n) -> Self {
                $n::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
            }
        }

        impl AddAssign for $n {
            #[inline]
            fn add_assign(&mut self, rhs: $n) {
                self.x += rhs.x;
                self.y += rhs.y;
                self.z += rhs.z;
            }
        }

        impl Sub for $n {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: $n) -> Self {
                $n::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
            }
        }

        impl SubAssign for $n {
            #[inline]
            fn sub_assign(&mut self, rhs: $n) {
                self.x -= rhs.x;
                self.y -= rhs.y;
                self.z -= rhs.z;
            }
        }

        impl Mul for $n {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: $n) -> Self {
                $n::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
            }
        }

        impl Mul<$n> for $t {
            type Output = $n;
            #[inline]
            fn mul(self, rhs: $n) -> $n {
                $n::new(self * rhs.x, self * rhs.y, self * rhs.z)
            }
        }

        impl Mul<$t> for $n {
            type Output = $n;
            #[inline]
            fn mul(self, rhs: $t) -> $n {
                $n::new(self.x * rhs, self.y * rhs, self.z * rhs)
            }
        }

        impl MulAssign for $n {
            #[inline]
            fn mul_assign(&mut self, rhs: $n) {
                self.x *= rhs.x;
                self.y *= rhs.y;
                self.z *= rhs.z;
            }
        }

        impl MulAssign<$t> for $n {
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                self.x *= rhs;
                self.y *= rhs;
                self.z *= rhs;
            }
        }

        impl Div for $n {
            type Output = Self;
            #[inline]
            fn div(self, rhs: $n) -> Self {
                $n::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
            }
        }

        impl Div<$t> for $n {
            type Output = $n;
            #[inline]
            fn div(self, rhs: $t) -> $n {
                $n::new(self.x / rhs, self.y / rhs, self.z / rhs)
            }
        }

        impl DivAssign for $n {
            #[inline]
            fn div_assign(&mut self, rhs: $n) {
                self.x /= rhs.x;
                self.y /= rhs.y;
                self.z /= rhs.z;
            }
        }

        impl DivAssign<$t> for $n {
            #[inline]
            fn div_assign(&mut self, rhs: $t) {
                self.x /= rhs;
                self.y /= rhs;
                self.z /= rhs;
            }
        }

        impl Neg for $n {
            type Output = $n;
            #[inline]
            fn neg(self) -> $n {
                self * $t::from(-1.0)
            }
        })+
    }
}

vec3s!((Vec3, Bivec3, Rotor3, Vec4) => f32, (Wec3, WBivec3, WRotor3, Wec4) => f32x4);

impl From<Vec2> for Vec3 {
    #[inline]
    fn from(vec: Vec2) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: 0.0,
        }
    }
}

impl From<Wec2> for Wec3 {
    #[inline]
    fn from(vec: Wec2) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: f32x4::from(0.0),
        }
    }
}

impl From<Vec4> for Vec3 {
    #[inline]
    fn from(vec: Vec4) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }
}

impl From<Wec4> for Wec3 {
    #[inline]
    fn from(vec: Wec4) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }
}

impl Vec3 {
    #[inline]
    pub fn refracted(&mut self, normal: Self, eta: f32) -> Self {
        let n = normal;
        let i = *self;
        let ndi = n.dot(i);
        let k = 1.0 - eta * eta * (1.0 - ndi * ndi);
        if k < 0.0 {
            Self::zero()
        } else {
            i * eta - n * (eta * ndi * k.sqrt())
        }
    }
}

impl Wec3 {
    #[inline]
    pub fn new_splat(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: f32x4::from(x),
            y: f32x4::from(y),
            z: f32x4::from(z),
        }
    }

    #[inline]
    pub fn splat(vec: Vec3) -> Self {
        Self::from([vec, vec, vec, vec])
    }

    #[inline]
    pub fn merge(mask: f32x4, a: Self, b: Self) -> Self {
        Self {
            x: f32x4::merge(mask, a.x, b.x),
            y: f32x4::merge(mask, a.y, b.y),
            z: f32x4::merge(mask, a.z, b.z),
        }
    }

    #[inline]
    pub fn refracted(&mut self, normal: Self, eta: f32x4) -> Self {
        let n = normal;
        let i = *self;
        let one = f32x4::from(1.0);
        let ndi = n.dot(i);

        let k = one - eta * eta * (one - ndi * ndi);
        let mask = k.cmp_lt(f32x4::from(0.0));

        let out = i * eta - n * (eta * ndi * k.sqrt());

        Self::merge(mask, out, Self::zero())
    }
}

impl Into<[Vec3; 4]> for Wec3 {
    #[inline]
    fn into(self) -> [Vec3; 4] {
        let xs = self.x.as_ref();
        let ys = self.y.as_ref();
        let zs = self.z.as_ref();
        [
            Vec3::new(xs[0], ys[0], zs[0]),
            Vec3::new(xs[1], ys[1], zs[1]),
            Vec3::new(xs[2], ys[2], zs[2]),
            Vec3::new(xs[3], ys[3], zs[3]),
        ]
    }
}

impl From<[Vec3; 4]> for Wec3 {
    #[inline]
    fn from(vecs: [Vec3; 4]) -> Self {
        Self {
            x: f32x4::new(vecs[0].x, vecs[1].x, vecs[2].x, vecs[3].x),
            y: f32x4::new(vecs[0].y, vecs[1].y, vecs[2].y, vecs[3].y),
            z: f32x4::new(vecs[0].z, vecs[1].z, vecs[2].z, vecs[3].z),
        }
    }
}

macro_rules! vec4s {
    ($($n:ident => $t:ident),+) => {
        /// A set of four coordinates which may be interpreted as a point or vector in 4d space,
        /// or as a homogeneous 3d vector or point.
        ///
        /// Generally this distinction between a point and vector is more of a pain than it is worth
        /// to distinguish on a type level, however when converting to and from homogeneous
        /// coordinates it is quite important.
        $(#[derive(Clone, Copy, Debug)]
        pub struct $n {
            pub x: $t,
            pub y: $t,
            pub z: $t,
            pub w: $t,
        }

        impl $n {
            #[inline]
            pub fn new<T: Into<$t>>(x: T, y: T, z: T, w: T) -> Self {
                $n { x: x.into(), y: y.into(), z: z.into(), w: w.into() }
            }

            #[inline]
            pub fn broadcast<T: Into<$t> + Copy>(val: T) -> Self {
                Self::new(val, val, val, val)
            }

            #[inline]
            pub fn unit_x() -> Self {
                $n{ x: $t::from(1.0), y: $t::from(0.0), z: $t::from(0.0), w: $t::from(0.0) }
            }

            #[inline]
            pub fn unit_y() -> Self {
                $n{ x: $t::from(0.0), y: $t::from(1.0), z: $t::from(0.0), w: $t::from(0.0) }
            }

            #[inline]
            pub fn unit_z() -> Self {
                $n{ x: $t::from(0.0), y: $t::from(0.0), z: $t::from(1.0), w: $t::from(0.0) }
            }

            #[inline]
            pub fn unit_w() -> Self {
                $n{ x: $t::from(0.0), y: $t::from(0.0), z: $t::from(0.0), w: $t::from(1.0) }
            }

            #[inline]
            pub fn dot(&self, other: $n) -> $t {
                self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
            }

            #[inline]
            pub fn reflect(&mut self, normal: $n) {
                *self -= $t::from(2.0) * self.dot(normal) * normal;
            }

            #[inline]
            pub fn reflected(&self, normal: $n) -> Self {
                let mut a = *self;
                a.reflect(normal);
                a
            }

            #[inline]
            pub fn mag_sq(&self) -> $t {
                self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
            }

            #[inline]
            pub fn mag(&self) -> $t {
                self.mag_sq().sqrt()
            }

            #[inline]
            pub fn normalize(&mut self) {
                let mag = self.mag();
                self.x /= mag;
                self.y /= mag;
                self.z /= mag;
                self.w /= mag;
            }

            #[inline]
            pub fn normalized(&self) -> Self {
                let mut r = self.clone();
                r.normalize();
                r
            }

            #[inline]
            pub fn mul_add(&self, mul: $n, add: $n) -> Self {
                $n::new(
                    self.x.mul_add(mul.x, add.x),
                    self.y.mul_add(mul.y, add.y),
                    self.z.mul_add(mul.z, add.z),
                    self.w.mul_add(mul.w, add.w),
                )
            }

            #[inline]
            pub fn abs(&self) -> Self {
                Self::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
            }

            #[inline]
            pub fn clamp(&mut self, min: Self, max: Self) {
                self.x = self.x.max(min.x).min(max.x);
                self.y = self.y.max(min.y).min(max.y);
                self.z = self.z.max(min.z).min(max.z);
                self.w = self.w.max(min.w).min(max.w);
            }

            #[inline]
            pub fn clamped(mut self, min: Self, max: Self) -> Self {
                self.clamp(min, max);
                self
            }

            #[inline]
            pub fn map<F>(&self, f: F) -> Self
                where F: Fn($t) -> $t
            {
                $n::new(
                    f(self.x),
                    f(self.y),
                    f(self.z),
                    f(self.w),
                )
            }

            #[inline]
            pub fn apply<F>(&mut self, f: F)
                where F: Fn($t) -> $t
            {
                self.x = f(self.x);
                self.y = f(self.y);
                self.z = f(self.z);
                self.w = f(self.w);
            }

            #[inline]
            pub fn max_by_component(mut self, other: Self) -> Self {
                self.x = self.x.max(other.x);
                self.y = self.y.max(other.y);
                self.z = self.z.max(other.z);
                self.w = self.w.max(other.w);
                self
            }

            #[inline]
            pub fn min_by_component(mut self, other: Self) -> Self {
                self.x = self.x.min(other.x);
                self.y = self.y.min(other.y);
                self.z = self.z.min(other.z);
                self.w = self.w.min(other.w);
                self
            }

            #[inline]
            pub fn component_max(&self) -> $t {
                self.x.max(self.y).max(self.z).max(self.w)
            }

            #[inline]
            pub fn component_min(&self) -> $t {
                self.x.min(self.y).min(self.z).min(self.w)
            }

            #[inline]
            pub fn zero() -> Self {
                Self::broadcast($t::from(0.0))
            }

            #[inline]
            pub fn one() -> Self {
                Self::broadcast($t::from(1.0))
            }
        }

        impl EqualsEps for $n {
            fn eq_eps(self, other: Self) -> bool {
                self.x.eq_eps(other.x) && self.y.eq_eps(other.y) && self.z.eq_eps(other.z) && self.w.eq_eps(other.w)
            }
        }

        impl From<[$t; 4]> for $n {
            #[inline]
            fn from(comps: [$t; 4]) -> Self {
                Self::new(comps[0], comps[1], comps[2], comps[3])
            }
        }

        impl Add for $n {
            type Output = Self;
            #[inline]
            fn add(self, rhs: $n) -> Self {
                $n::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
            }
        }

        impl AddAssign for $n {
            #[inline]
            fn add_assign(&mut self, rhs: $n) {
                self.x += rhs.x;
                self.y += rhs.y;
                self.z += rhs.z;
                self.w += rhs.w;
            }
        }

        impl Sub for $n {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: $n) -> Self {
                $n::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
            }
        }

        impl SubAssign for $n {
            #[inline]
            fn sub_assign(&mut self, rhs: $n) {
                self.x -= rhs.x;
                self.y -= rhs.y;
                self.z -= rhs.z;
                self.w -= rhs.w;
            }
        }

        impl Mul for $n {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: $n) -> Self {
                $n::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z, self.w * rhs. w)
            }
        }

        impl Mul<$n> for $t {
            type Output = $n;
            #[inline]
            fn mul(self, rhs: $n) -> $n {
                $n::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
            }
        }

        impl Mul<$t> for $n {
            type Output = $n;
            #[inline]
            fn mul(self, rhs: $t) -> $n {
                $n::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
            }
        }

        impl MulAssign for $n {
            #[inline]
            fn mul_assign(&mut self, rhs: $n) {
                self.x *= rhs.x;
                self.y *= rhs.y;
                self.z *= rhs.z;
                self.w *= rhs.w;
            }
        }

        impl MulAssign<$t> for $n {
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                self.x *= rhs;
                self.y *= rhs;
                self.z *= rhs;
                self.w *= rhs;
            }
        }

        impl Div for $n {
            type Output = Self;
            #[inline]
            fn div(self, rhs: $n) -> Self {
                $n::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z, self.w / rhs.w)
            }
        }

        impl Div<$t> for $n {
            type Output = $n;
            #[inline]
            fn div(self, rhs: $t) -> $n {
                $n::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
            }
        }

        impl DivAssign for $n {
            #[inline]
            fn div_assign(&mut self, rhs: $n) {
                self.x /= rhs.x;
                self.y /= rhs.y;
                self.z /= rhs.z;
                self.w /= rhs.w;
            }
        }

        impl DivAssign<$t> for $n {
            #[inline]
            fn div_assign(&mut self, rhs: $t) {
                self.x /= rhs;
                self.y /= rhs;
                self.z /= rhs;
                self.w /= rhs;
            }
        }

        impl Neg for $n {
            type Output = $n;
            #[inline]
            fn neg(self) -> $n {
                self * $t::from(-1.0)
            }
        })+
    }
}

vec4s!(Vec4 => f32, Wec4 => f32x4);

impl From<Vec3> for Vec4 {
    #[inline]
    fn from(vec: Vec3) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.x,
            w: 0.0,
        }
    }
}

impl From<Wec3> for Wec4 {
    #[inline]
    fn from(vec: Wec3) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
            w: f32x4::from(0.0),
        }
    }
}

impl Vec4 {
    #[inline]
    pub fn refracted(&mut self, normal: Self, eta: f32) -> Self {
        let n = normal;
        let i = *self;
        let ndi = n.dot(i);
        let k = 1.0 - eta * eta * (1.0 - ndi * ndi);
        if k < 0.0 {
            Self::zero()
        } else {
            i * eta - n * (eta * ndi * k.sqrt())
        }
    }
}

impl Wec4 {
    #[inline]
    pub fn new_splat(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x: f32x4::from(x),
            y: f32x4::from(y),
            z: f32x4::from(z),
            w: f32x4::from(w),
        }
    }

    #[inline]
    pub fn splat(vec: Vec4) -> Self {
        Self::from([vec, vec, vec, vec])
    }

    #[inline]
    pub fn merge(mask: f32x4, a: Self, b: Self) -> Self {
        Self {
            x: f32x4::merge(mask, a.x, b.x),
            y: f32x4::merge(mask, a.y, b.y),
            z: f32x4::merge(mask, a.z, b.z),
            w: f32x4::merge(mask, a.w, b.w),
        }
    }

    #[inline]
    pub fn refracted(&mut self, normal: Self, eta: f32x4) -> Self {
        let n = normal;
        let i = *self;
        let one = f32x4::from(1.0);
        let ndi = n.dot(i);

        let k = one - eta * eta * (one - ndi * ndi);
        let mask = k.cmp_lt(f32x4::from(0.0));

        let out = i * eta - n * (eta * ndi * k.sqrt());

        Self::merge(mask, out, Self::zero())
    }
}

impl Into<[Vec4; 4]> for Wec4 {
    #[inline]
    fn into(self) -> [Vec4; 4] {
        let xs = self.x.as_ref();
        let ys = self.y.as_ref();
        let zs = self.z.as_ref();
        let ws = self.w.as_ref();
        [
            Vec4::new(xs[0], ys[0], zs[0], ws[0]),
            Vec4::new(xs[1], ys[1], zs[1], ws[1]),
            Vec4::new(xs[2], ys[2], zs[2], ws[2]),
            Vec4::new(xs[3], ys[3], zs[3], ws[3]),
        ]
    }
}

impl From<[Vec4; 4]> for Wec4 {
    #[inline]
    fn from(vecs: [Vec4; 4]) -> Self {
        Self {
            x: f32x4::new(vecs[0].x, vecs[1].x, vecs[2].x, vecs[3].x),
            y: f32x4::new(vecs[0].y, vecs[1].y, vecs[2].y, vecs[3].y),
            z: f32x4::new(vecs[0].z, vecs[1].z, vecs[2].z, vecs[3].z),
            w: f32x4::new(vecs[0].w, vecs[1].w, vecs[2].w, vecs[3].w),
        }
    }
}
