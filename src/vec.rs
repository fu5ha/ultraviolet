//! Vectors and points, i.e. directed line segments and locations.
use crate::bivec::*;
use crate::rotor::*;
use crate::util::*;
use std::ops::*;

use wide::f32x4;

#[cfg(feature = "serde")]
use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};

macro_rules! vec2s {
    ($(($n:ident, $bn:ident, $rn:ident, $v3t:ident, $v4t:ident) => $t:ident),+) => {
        $(
        /// A set of two coordinates which may be interpreted as a vector or point in 2d space.
        ///
        /// Generally this distinction between a point and vector is more of a pain than it is worth
        /// to distinguish on a type level, however when converting to and from homogeneous
        /// coordinates it is quite important.
        #[derive(Clone, Copy, Debug, Default)]
        #[repr(C)]
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
                self.x.mul_add(other.x, self.y * other.y)
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
                $bn::new(self.x.mul_add(other.y, -(other.x * self.y)))
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
                self.x.mul_add(self.x, self.y * self.y)
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
                    self.y.mul_add(mul.y, add.y),
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

            #[inline]
            pub fn xyz(&self) -> $v3t {
                $v3t::new(self.x, self.y, $t::from(0.0))
            }

            #[inline]
            pub fn xyzw(&self) -> $v4t {
                $v4t::new(self.x, self.y, $t::from(0.0), $t::from(0.0))
            }

            #[inline]
            pub fn layout() -> alloc::alloc::Layout {
                alloc::alloc::Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<$t>()).unwrap()
            }

            #[inline]
            pub fn as_array(&self) -> &[$t; 2] {
                use std::convert::TryInto;
                self.as_slice().try_into().unwrap()
            }

            #[inline]
            pub fn as_slice(&self) -> &[$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const $t, 2)
                }
            }


            #[inline]
            pub fn as_byte_slice(&self) -> &[u8] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const u8, 2 * std::mem::size_of::<$t>())
                }
            }

            #[inline]
            pub fn as_mut_slice(&mut self) -> &mut [$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(self as *mut $n as *mut $t, 2)
                }
            }

            #[inline]
            pub fn as_mut_byte_slice(&mut self) -> &mut [u8] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(self as *mut $n as *mut u8, 2 * std::mem::size_of::<$t>())
                }
            }

            /// Returns a constant unsafe pointer to the underlying data in the underlying type.
            /// This function is safe because all types here are repr(C) and can be represented
            /// as their underlying type.
            ///
            /// # Safety
            ///
            /// It is up to the caller to correctly use this pointer and its bounds.
            #[inline]
            pub fn as_ptr(&self) -> *const $t {
                self as *const $n as *const $t
            }

            /// Returns a mutable unsafe pointer to the underlying data in the underlying type.
            /// This function is safe because all types here are repr(C) and can be represented
            /// as their underlying type.
            ///
            /// # Safety
            ///
            /// It is up to the caller to correctly use this pointer and its bounds.
            #[inline]
            pub fn as_mut_ptr(&mut self) -> *mut $t {
                self as *mut $n as *mut $t
            }
        }

        impl Into<[$t; 2]> for $n {
            #[inline]
            fn into(self) -> [$t; 2] {
                [self.x, self.y]
            }
        }

        impl From<[$t; 2]> for $n {
            #[inline]
            fn from(comps: [$t; 2]) -> Self {
                Self::new(comps[0], comps[1])
            }
        }

        impl From<&[$t; 2]> for $n {
            #[inline]
            fn from(comps: &[$t; 2]) -> Self {
                Self::from(*comps)
            }
        }

        impl From<&mut [$t; 2]> for $n {
            #[inline]
            fn from(comps: &mut [$t; 2]) -> Self {
                Self::from(*comps)
            }
        }

        impl From<($t, $t)> for $n {
            #[inline]
            fn from(comps: ($t, $t)) -> Self {
                Self::new(comps.0, comps.1)
            }
        }

        impl From<&($t, $t)> for $n {
            #[inline]
            fn from(comps: &($t, $t)) -> Self {
                Self::from(*comps)
            }
        }

        impl From<$n> for ($t, $t) {
            #[inline]
            fn from(v: $n) -> Self {
                (v.x, v.y)
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
        }

        impl Index<usize> for $n {
            type Output = $t;

            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    _ => panic!("Invalid for vector of type: {}", std::any::type_name::<$n>()),
                }
            }
        }

        impl IndexMut<usize> for $n {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    _ => panic!("Invalid for vector of type: {}", std::any::type_name::<$n>()),
                }
            }
        }
        )+
    };
}

vec2s!((Vec2, Bivec2, Rotor2, Vec3, Vec4) => f32, (Wec2, WBivec2, WRotor2, Wec3, Wec4) => f32x4);

impl From<[Vec2; 4]> for Wec2 {
    #[inline]
    fn from(vecs: [Vec2; 4]) -> Self {
        Self {
            x: f32x4::from([vecs[0].x, vecs[1].x, vecs[2].x, vecs[3].x]),
            y: f32x4::from([vecs[0].y, vecs[1].y, vecs[2].y, vecs[3].y]),
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
    pub fn refract(&mut self, normal: Self, eta: f32) {
        *self = self.refracted(normal, eta);
    }

    #[inline]
    pub fn refracted(&self, normal: Self, eta: f32) -> Self {
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

    /// Merge two vectors together lanewise using `mask` as a mask.
    ///
    /// This is essentially a bitwise merge operation, such that any point where
    /// there is a 1 bit in `mask`, the output will put the bit from `tru`, while
    /// where there is a 0 bit in `mask`, the output will put the bit from `fals`
    #[inline]
    pub fn merge(mask: f32x4, tru: Self, fals: Self) -> Self {
        Self {
            x: mask.merge(tru.x, fals.x),
            y: mask.merge(tru.y, fals.y),
        }
    }

    #[inline]
    pub fn refract(&mut self, normal: Self, eta: f32x4) {
        *self = self.refracted(normal, eta);
    }

    #[inline]
    pub fn refracted(&self, normal: Self, eta: f32x4) -> Self {
        let n = normal;
        let i = *self;
        let one = f32x4::from(1.0);
        let ndi = n.dot(i);

        let k = one - eta * eta * (one - ndi * ndi);
        let mask = k.cmp_lt(f32x4::from(0.0));

        let out = i * eta - n * (eta * ndi * k.sqrt());

        Self::merge(mask, Self::zero(), out)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Vec2 {
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        let mut state = serializer.serialize_struct("Vec2", 2)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Vec2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            X,
            Y,
        };

        // This part could also be generated independently by:
        //
        //    #[derive(Deserialize)]
        //    #[serde(field_identifier, rename_all = "lowercase")]
        //    enum Field { Secs, Nanos }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`x` or `y`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "x" => Ok(Field::X),
                            "y" => Ok(Field::Y),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct Vec2Visitor;

        impl<'de> Visitor<'de> for Vec2Visitor {
            type Value = Vec2;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Vec2")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Vec2, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let x = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let y = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                Ok(Vec2::new(x, y))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Vec2, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut x = None;
                let mut y = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::X => {
                            if x.is_some() {
                                return Err(serde::de::Error::duplicate_field("x"));
                            }
                            x = Some(map.next_value()?);
                        }
                        Field::Y => {
                            if y.is_some() {
                                return Err(serde::de::Error::duplicate_field("y"));
                            }
                            y = Some(map.next_value()?);
                        }
                    }
                }
                let x = x.ok_or_else(|| serde::de::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| serde::de::Error::missing_field("y"))?;
                Ok(Vec2::new(x, y))
            }
        }

        const FIELDS: &'static [&'static str] = &["x", "y"];

        deserializer.deserialize_struct("Vec2", FIELDS, Vec2Visitor)
    }
}

macro_rules! vec3s {
    ($(($v2t:ident, $n:ident, $bn:ident, $rn:ident, $v4t:ident) => $t:ident),+) => {
        /// A set of three coordinates which may be interpreted as a point or vector in 3d space,
        /// or as a homogeneous 2d vector or point.
        ///
        /// Generally this distinction between a point and vector is more of a pain than it is worth
        /// to distinguish on a type level, however when converting to and from homogeneous
        /// coordinates it is quite important.
        $(#[derive(Clone, Copy, Debug, Default)]
        #[repr(C)]
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
                self.x.mul_add(other.x, self.y.mul_add(other.y, self.z * other.z))
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
                    self.x.mul_add(other.y, -(self.y * other.x)),
                    self.x.mul_add(other.z, -(self.z * other.x)),
                    self.y.mul_add(other.z, -(self.z * other.y)),
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
                    self.y.mul_add(other.z, -self.z * other.y),
                    self.z.mul_add(other.x, -self.x * other.z),
                    self.x.mul_add(other.y, -self.y * other.x),
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
                self.x.mul_add(self.x, self.y.mul_add(self.y, self.z * self.z))
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


            #[inline]
            pub fn xy(&self) -> $v2t {
                $v2t::new(self.x, self.y)
            }

            #[inline]
            pub fn xyzw(&self) -> $v4t {
                $v4t::new(self.x, self.y, self.z, $t::from(0.0))
            }

            #[inline]
            pub fn layout() -> alloc::alloc::Layout {
                alloc::alloc::Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<$t>()).unwrap()
            }

            #[inline]
            pub fn as_array(&self) -> &[$t; 3] {
                use std::convert::TryInto;
                self.as_slice().try_into().unwrap()
            }

            #[inline]
            pub fn as_slice(&self) -> &[$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const $t, 3)
                }
            }

            #[inline]
            pub fn as_byte_slice(&self) -> &[u8] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const u8, 3 * std::mem::size_of::<$t>())
                }
            }

            #[inline]
            pub fn as_mut_slice(&mut self) -> &mut [$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(self as *mut $n as *mut $t, 3)
                }
            }

            #[inline]
            pub fn as_mut_byte_slice(&mut self) -> &mut [u8] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(self as *mut $n as *mut u8, 3 * std::mem::size_of::<$t>())
                }
            }

            /// Returns a constant unsafe pointer to the underlying data in the underlying type.
            /// This function is safe because all types here are repr(C) and can be represented
            /// as their underlying type.
            ///
            /// # Safety
            ///
            /// It is up to the caller to correctly use this pointer and its bounds.
            #[inline]
            pub fn as_ptr(&self) -> *const $t {
                self as *const $n as *const $t
            }

            /// Returns a mutable unsafe pointer to the underlying data in the underlying type.
            /// This function is safe because all types here are repr(C) and can be represented
            /// as their underlying type.
            ///
            /// # Safety
            ///
            /// It is up to the caller to correctly use this pointer and its bounds.
            #[inline]
            pub fn as_mut_ptr(&mut self) -> *mut $t {
                self as *mut $n as *mut $t
            }
        }

        impl EqualsEps for $n {
            fn eq_eps(self, other: Self) -> bool {
                self.x.eq_eps(other.x) && self.y.eq_eps(other.y) && self.z.eq_eps(other.z)
            }
        }

        impl Into<[$t; 3]> for $n {
            #[inline]
            fn into(self) -> [$t; 3] {
                [self.x, self.y, self.z]
            }
        }

        impl From<[$t; 3]> for $n {
            #[inline]
            fn from(comps: [$t; 3]) -> Self {
                Self::new(comps[0], comps[1], comps[2])
            }
        }

        impl From<&[$t; 3]> for $n {
            #[inline]
            fn from(comps: &[$t; 3]) -> Self {
               Self::from(*comps)
            }
        }

        impl From<&mut [$t; 3]> for $n {
            #[inline]
            fn from(comps: &mut [$t; 3]) -> Self {
                Self::from(*comps)
            }
        }

       impl From<($t, $t, $t)> for $n {
            #[inline]
            fn from(comps: ($t, $t, $t)) -> Self {
                Self::new(comps.0, comps.1, comps.2)
            }
        }

        impl From<&($t, $t, $t)> for $n {
            #[inline]
            fn from(comps: &($t, $t, $t)) -> Self {
                Self::from(*comps)
            }
        }

        impl From<$n> for ($t, $t, $t) {
            #[inline]
            fn from(v: $n) -> Self {
                (v.x, v.y, v.z)
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
        }

        impl Index<usize> for $n {
            type Output = $t;

            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    2 => &self.z,
                    _ => panic!("Invalid for vector of type: {}", std::any::type_name::<$n>()),
                }
            }
        }

        impl IndexMut<usize> for $n {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    2 => &mut self.z,
                    _ => panic!("Invalid for vector of type: {}", std::any::type_name::<$n>()),
                }
            }
        }
        )+
    }
}

vec3s!((Vec2, Vec3, Bivec3, Rotor3, Vec4) => f32, (Wec2, Wec3, WBivec3, WRotor3, Wec4) => f32x4);

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
    pub fn refract(&mut self, normal: Self, eta: f32) {
        *self = self.refracted(normal, eta);
    }

    #[inline]
    pub fn refracted(&self, normal: Self, eta: f32) -> Self {
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

    /// Merge two vectors together lanewise using `mask` as a mask.
    ///
    /// This is essentially a bitwise merge operation, such that any point where
    /// there is a 1 bit in `mask`, the output will put the bit from `tru`, while
    /// where there is a 0 bit in `mask`, the output will put the bit from `fals`
    #[inline]
    pub fn merge(mask: f32x4, tru: Self, fals: Self) -> Self {
        Self {
            x: mask.merge(tru.x, fals.x),
            y: mask.merge(tru.y, fals.y),
            z: mask.merge(tru.z, fals.z),
        }
    }

    #[inline]
    pub fn refract(&mut self, normal: Self, eta: f32x4) {
        *self = self.refracted(normal, eta);
    }

    #[inline]
    pub fn refracted(&self, normal: Self, eta: f32x4) -> Self {
        let n = normal;
        let i = *self;
        let one = f32x4::from(1.0);
        let ndi = n.dot(i);

        let k = one - eta * eta * (one - ndi * ndi);
        let mask = k.cmp_lt(f32x4::from(0.0));

        let out = i.mul_add(Wec3::broadcast(eta), -n * (eta * ndi * k.sqrt()));

        Self::merge(mask, Self::zero(), out)
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
            x: f32x4::from([vecs[0].x, vecs[1].x, vecs[2].x, vecs[3].x]),
            y: f32x4::from([vecs[0].y, vecs[1].y, vecs[2].y, vecs[3].y]),
            z: f32x4::from([vecs[0].z, vecs[1].z, vecs[2].z, vecs[3].z]),
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for Vec3 {
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        let mut state = serializer.serialize_struct("Vec3", 3)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.serialize_field("z", &self.z)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Vec3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            X,
            Y,
            Z,
        };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`x` or `y` or `z`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "x" => Ok(Field::X),
                            "y" => Ok(Field::Y),
                            "z" => Ok(Field::Z),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct Vec3Visitor;

        impl<'de> Visitor<'de> for Vec3Visitor {
            type Value = Vec3;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Vec3")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Vec3, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let x = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let y = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let z = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                Ok(Vec3::new(x, y, z))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Vec3, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut x = None;
                let mut y = None;
                let mut z = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::X => {
                            if x.is_some() {
                                return Err(serde::de::Error::duplicate_field("x"));
                            }
                            x = Some(map.next_value()?);
                        }
                        Field::Y => {
                            if y.is_some() {
                                return Err(serde::de::Error::duplicate_field("y"));
                            }
                            y = Some(map.next_value()?);
                        }
                        Field::Z => {
                            if z.is_some() {
                                return Err(serde::de::Error::duplicate_field("z"));
                            }
                            z = Some(map.next_value()?);
                        }
                    }
                }
                let x = x.ok_or_else(|| serde::de::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| serde::de::Error::missing_field("y"))?;
                let z = z.ok_or_else(|| serde::de::Error::missing_field("z"))?;
                Ok(Vec3::new(x, y, z))
            }
        }

        const FIELDS: &'static [&'static str] = &["x", "y", "z"];

        deserializer.deserialize_struct("Vec3", FIELDS, Vec3Visitor)
    }
}

macro_rules! vec4s {
    ($($n:ident, $v2t:ident, $v3t:ident => $t:ident),+) => {
        /// A set of four coordinates which may be interpreted as a point or vector in 4d space,
        /// or as a homogeneous 3d vector or point.
        ///
        /// Generally this distinction between a point and vector is more of a pain than it is worth
        /// to distinguish on a type level, however when converting to and from homogeneous
        /// coordinates it is quite important.
        $(#[derive(Clone, Copy, Debug, Default)]
        #[repr(C)]
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
                self.x.mul_add(other.x, self.y.mul_add(other.y, self.z.mul_add(other.z, self.w * other.w)))
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
                self.x.mul_add(self.x, self.y.mul_add(self.y, self.z.mul_add(self.z, self.w * self.w)))
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

            #[inline]
            pub fn xy(&self) -> $v2t {
                $v2t::new(self.x, self.y)
            }

             #[inline]
            pub fn xyz(&self) -> $v3t {
                $v3t::new(self.x, self.y, self.z)
            }


            #[inline]
            pub fn layout() -> alloc::alloc::Layout {
                alloc::alloc::Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<$t>()).unwrap()
            }

            #[inline]
            pub fn as_array(&self) -> &[$t; 4] {
                use std::convert::TryInto;
                self.as_slice().try_into().unwrap()
            }

            #[inline]
            pub fn as_slice(&self) -> &[$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const $t, 4)
                }
            }

            #[inline]
            pub fn as_byte_slice(&self) -> &[u8] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const u8, 4 * std::mem::size_of::<$t>())
                }
            }

            #[inline]
            pub fn as_mut_slice(&mut self) -> &mut [$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(self as *mut $n as *mut $t, 4)
                }
            }

            #[inline]
            pub fn as_mut_byte_slice(&mut self) -> &mut [u8] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(self as *mut $n as *mut u8, 4 * std::mem::size_of::<$t>())
                }
            }

            /// Returns a constant unsafe pointer to the underlying data in the underlying type.
            /// This function is safe because all types here are repr(C) and can be represented
            /// as their underlying type.
            ///
            /// # Safety
            ///
            /// It is up to the caller to correctly use this pointer and its bounds.
            #[inline]
            pub fn as_ptr(&self) -> *const $t {
                 self as *const $n as *const $t
            }

            /// Returns a mutable unsafe pointer to the underlying data in the underlying type.
            /// This function is safe because all types here are repr(C) and can be represented
            /// as their underlying type.
            ///
            /// # Safety
            ///
            /// It is up to the caller to correctly use this pointer and its bounds.
            #[inline]
            pub fn as_mut_ptr(&mut self) -> *mut $t {
                self as *mut $n as *mut $t
            }
        }

        impl EqualsEps for $n {
            fn eq_eps(self, other: Self) -> bool {
                self.x.eq_eps(other.x) && self.y.eq_eps(other.y) && self.z.eq_eps(other.z) && self.w.eq_eps(other.w)
            }
        }

        impl Into<[$t; 4]> for $n {
            #[inline]
            fn into(self) -> [$t; 4] {
                [self.x, self.y, self.z, self.w]
            }
        }

        impl From<[$t; 4]> for $n {
            #[inline]
            fn from(comps: [$t; 4]) -> Self {
                Self::new(comps[0], comps[1], comps[2], comps[3])
            }
        }

        impl From<&[$t; 4]> for $n {
            #[inline]
            fn from(comps: &[$t; 4]) -> Self {
                Self::from(*comps)
            }
        }

        impl From<&mut [$t; 4]> for $n {
            #[inline]
            fn from(comps: &mut [$t; 4]) -> Self {
                Self::from(*comps)
            }
        }

        impl From<($t, $t, $t, $t)> for $n {
            #[inline]
            fn from(comps: ($t, $t, $t, $t)) -> Self {
                Self::new(comps.0, comps.1, comps.2, comps.3)
            }
        }

        impl From<&($t, $t, $t, $t)> for $n {
            #[inline]
            fn from(comps: &($t, $t, $t, $t)) -> Self {
                Self::from(*comps)
            }
        }

        impl From<$n> for ($t, $t, $t, $t) {
            #[inline]
            fn from(v: $n) -> Self {
                (v.x, v.y, v.z, v.w)
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
        }

        impl Index<usize> for $n {
            type Output = $t;

            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    2 => &self.z,
                    3 => &self.w,
                    _ => panic!("Invalid for vector of type: {}", std::any::type_name::<$n>()),
                }
            }
        }

        impl IndexMut<usize> for $n {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    2 => &mut self.z,
                    3 => &mut self.w,
                    _ => panic!("Invalid for vector of type: {}", std::any::type_name::<$n>()),
                }
            }
        }
        )+
    }
}

vec4s!(Vec4, Vec2, Vec3 => f32, Wec4, Wec2, Wec3 => f32x4);

impl From<Vec3> for Vec4 {
    #[inline]
    fn from(vec: Vec3) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
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
    pub fn refract(&mut self, normal: Self, eta: f32) {
        *self = self.refracted(normal, eta);
    }

    #[inline]
    pub fn refracted(&self, normal: Self, eta: f32) -> Self {
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

    /// Merge two vectors together lanewise using `mask` as a mask.
    ///
    /// This is essentially a bitwise merge operation, such that any point where
    /// there is a 1 bit in `mask`, the output will put the bit from `tru`, while
    /// where there is a 0 bit in `mask`, the output will put the bit from `fals`
    #[inline]
    pub fn merge(mask: f32x4, tru: Self, fals: Self) -> Self {
        Self {
            x: mask.merge(tru.x, fals.x),
            y: mask.merge(tru.y, fals.y),
            z: mask.merge(tru.z, fals.z),
            w: mask.merge(tru.w, fals.w),
        }
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
            x: f32x4::from([vecs[0].x, vecs[1].x, vecs[2].x, vecs[3].x]),
            y: f32x4::from([vecs[0].y, vecs[1].y, vecs[2].y, vecs[3].y]),
            z: f32x4::from([vecs[0].z, vecs[1].z, vecs[2].z, vecs[3].z]),
            w: f32x4::from([vecs[0].w, vecs[1].w, vecs[2].w, vecs[3].w]),
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for Vec4 {
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        let mut state = serializer.serialize_struct("Vec4", 3)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.serialize_field("z", &self.z)?;
        state.serialize_field("w", &self.w)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Vec4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            X,
            Y,
            Z,
            W,
        };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`x` or `y` or `z` or `w`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "x" => Ok(Field::X),
                            "y" => Ok(Field::Y),
                            "z" => Ok(Field::Z),
                            "w" => Ok(Field::W),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct Vec4Visitor;

        impl<'de> Visitor<'de> for Vec4Visitor {
            type Value = Vec4;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Vec4")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Vec4, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let x = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let y = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let z = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                let w: f32 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(3, &self))?;
                Ok(Vec4::new(x, y, z, w))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Vec4, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut x = None;
                let mut y = None;
                let mut z = None;
                let mut w = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::X => {
                            if x.is_some() {
                                return Err(serde::de::Error::duplicate_field("x"));
                            }
                            x = Some(map.next_value()?);
                        }
                        Field::Y => {
                            if y.is_some() {
                                return Err(serde::de::Error::duplicate_field("y"));
                            }
                            y = Some(map.next_value()?);
                        }
                        Field::Z => {
                            if z.is_some() {
                                return Err(serde::de::Error::duplicate_field("z"));
                            }
                            z = Some(map.next_value()?);
                        }
                        Field::W => {
                            if w.is_some() {
                                return Err(serde::de::Error::duplicate_field("w"));
                            }
                            w = Some(map.next_value()?);
                        }
                    }
                }
                let x = x.ok_or_else(|| serde::de::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| serde::de::Error::missing_field("y"))?;
                let z = z.ok_or_else(|| serde::de::Error::missing_field("z"))?;
                let w: f32 = w.ok_or_else(|| serde::de::Error::missing_field("w"))?;
                Ok(Vec4::new(x, y, z, w))
            }
        }

        const FIELDS: &'static [&'static str] = &["x", "y", "z", "w"];

        deserializer.deserialize_struct("Vec4", FIELDS, Vec4Visitor)
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use crate::vec::{Vec2, Vec3, Vec4};
    use serde_test::{assert_tokens, Token};

    #[test]
    fn vec2() {
        let vec2 = Vec2::new(1.0, 2.0);

        //        @TODO we need PartialEq for this...
        //        assert_tokens(&vec2, &[
        //            Token::F32(1.0),
        //            Token::F32(2.0),
        //        ]);
    }

    #[test]
    fn vec3() {
        let vec3 = Vec3::new(1.0, 2.0, 3.0);

        //        @TODO we need PartialEq for this...
        //        assert_tokens(&vec2, &[
        //            Token::F32(1.0),
        //            Token::F32(2.0),
        //            Token::F32(3.0),
        //        ]);
    }

    #[test]
    fn vec4() {
        let vec4 = Vec4::new(1.0, 2.0, 3.0, 4.0);

        //        @TODO we need PartialEq for this...
        //        assert_tokens(&vec2, &[
        //            Token::F32(1.0),
        //            Token::F32(2.0),
        //            Token::F32(3.0),
        //            Token::F32(4.0),
        //        ]);
    }
}