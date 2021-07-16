use crate::*;
use std::convert::{TryFrom, TryInto};
use std::ops::*;

pub trait MulAdd<A = Self, B = Self> {
    /// The resulting type after applying the fused multiply-add.
    type Output;

    /// Performs the fused multiply-add operation.
    fn mul_add(self, a: A, b: B) -> Self::Output;
}

impl MulAdd<u32, u32> for u32 {
    type Output = u32;

    fn mul_add(self, a: u32, b: u32) -> Self::Output {
        (self * a) + b
    }
}

impl MulAdd<i32, i32> for i32 {
    type Output = i32;

    fn mul_add(self, a: i32, b: i32) -> Self::Output {
        (self * a) + b
    }
}

macro_rules! ivec2s {
    ($(($n:ident, $v3t:ident, $v4t:ident) => $t:ident),+) => {
        $(
        /// A set of two coordinates which may be interpreted as a vector or point in 2d space.
        ///
        /// Generally this distinction between a point and vector is more of a pain than it is worth
        /// to distinguish on a type level, however when converting to and from homogeneous
        /// coordinates it is quite important.
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
        #[repr(C)]
        pub struct $n {
            pub x: $t,
            pub y: $t,
        }


        impl $n {
            #[inline]
            pub const fn new(x: $t, y: $t) -> Self {
                $n { x, y }
            }

            #[inline]
            pub fn broadcast(val: $t) -> Self {
                Self::new(val, val)
            }

            #[inline]
            pub fn unit_x() -> Self {
                $n{ x: 1, y: 0 }
            }

            #[inline]
            pub fn unit_y() -> Self {
                $n{ x: 0, y: 1 }
            }

            /// Create a homogeneous 2d *point* from this vector interpreted as a point,
            /// meaning the homogeneous component will start with a value of 1.
            #[inline]
            pub fn into_homogeneous_point(self) -> $v3t {
                $v3t { x: self.x, y: self.y, z: 1 }
            }

            /// Create a homogeneous 2d *vector* from this vector,
            /// meaning the homogeneous component will always have a value of 0.
            #[inline]
            pub fn into_homogeneous_vector(self) -> $v3t {
                $v3t { x: self.x, y: self.y, z: 0 }
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
                (self.x * other.x) + (self.y * other.y)
            }

            #[inline]
            pub fn reflected(&self, normal: $n) -> Self {
                *self - (2 * self.dot(normal) * normal)
            }

            #[inline]
            pub fn mag(&self) -> $t {
                (self.mag_sq() as f64).sqrt() as $t
            }

            #[inline]
            pub fn mag_sq(&self) -> $t {
                (self.x * self.x) + (self.y * self.y)
            }

            #[inline]
            pub fn mul_add(&self, mul: $n, add: $n) -> Self {
                $n::new(
                    self.x.mul_add(mul.x, add.x),
                    self.y.mul_add(mul.y, add.y),
                )
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
                Self::broadcast(0)
            }

            #[inline]
            pub fn one() -> Self {
                Self::broadcast(1)
            }

            #[inline]
            pub fn xyz(&self) -> $v3t {
                $v3t::new(self.x, self.y, 0)
            }

            #[inline]
            pub fn xyzw(&self) -> $v4t {
                $v4t::new(self.x, self.y, 0, 0)
            }

            #[inline]
            pub fn layout() -> alloc::alloc::Layout {
                alloc::alloc::Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<$t>()).unwrap()
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
            pub fn as_array(&self) -> [$t; 2] {
                use std::convert::TryInto;
                self.as_slice().try_into().unwrap()
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

        impl From<[$t; 2]> for $n {
            #[inline]
            fn from(comps: [$t; 2]) -> Self {
                Self::new(comps[0], comps[1])
            }
        }

        impl From<$n> for [$t; 2] {
            #[inline]
            fn from(v: $n) -> Self {
                [v.x, v.y]
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

macro_rules! ivec3s {
    ($(($v2t:ident, $n:ident, $v4t:ident) => $t:ident),+) => {
        /// A set of three coordinates which may be interpreted as a point or vector in 3d space,
        /// or as a homogeneous 2d vector or point.
        ///
        /// Generally this distinction between a point and vector is more of a pain than it is worth
        /// to distinguish on a type level, however when converting to and from homogeneous
        /// coordinates it is quite important.
        $(#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
        #[repr(C)]
        pub struct $n {
            pub x: $t,
            pub y: $t,
            pub z: $t,
        }

        impl $n {
            #[inline]
            pub const fn new(x: $t, y: $t, z: $t) -> Self {
                $n { x, y, z }
            }

            #[inline]
            pub fn broadcast(val: $t) -> Self {
                Self::new(val, val, val)
            }

            #[inline]
            pub fn unit_x() -> Self {
                $n{ x: 1, y: 0, z: 0 }
            }

            #[inline]
            pub fn unit_y() -> Self {
                $n{ x: 0, y: 1, z: 0 }
            }

            #[inline]
            pub fn unit_z() -> Self {
                $n{ x: 0, y: 0, z: 1 }
            }

            #[inline]
            pub fn cross(&self, other: $n) -> Self {
                $n::new(
                    self.y.mul_add(other.z, -(self.z as i32) as $t * other.y),
                    self.z.mul_add(other.x, -(self.x as i32) as $t * other.z),
                    self.x.mul_add(other.y, -(self.y as i32) as $t * other.x),
                )
            }

            /// Create a homogeneous 3d *point* from this vector interpreted as a point,
            /// meaning the homogeneous component will start with a value of 1.
            #[inline]
            pub fn into_homogeneous_point(self) -> $v4t {
                $v4t { x: self.x, y: self.y, z: self.z, w: 1 }
            }

            /// Create a homogeneous 3d *vector* from this vector,
            /// meaning the homogeneous component will always have a value of 0.
            #[inline]
            pub fn into_homogeneous_vector(self) -> $v4t {
                $v4t { x: self.x, y: self.y, z: self.z, w: 0 }
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
                (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
            }

            #[inline]
            pub fn reflect(&mut self, normal: $n) {
                *self -= 2 * self.dot(normal) * normal;
            }

            #[inline]
            pub fn reflected(&self, normal: $n) -> Self {
                let mut a = *self;
                a.reflect(normal);
                a
            }

            #[inline]
            pub fn mag(&self) -> $t {
                (self.mag_sq() as f64).sqrt() as $t
            }

            #[inline]
            pub fn mag_sq(&self) -> $t {
                (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
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
                Self::broadcast(0)
            }

            #[inline]
            pub fn one() -> Self {
                Self::broadcast(1)
            }


            #[inline]
            pub fn xy(&self) -> $v2t {
                $v2t::new(self.x, self.y)
            }

            #[inline]
            pub fn xyzw(&self) -> $v4t {
                $v4t::new(self.x, self.y, self.z, 0)
            }

            #[inline]
            pub fn layout() -> alloc::alloc::Layout {
                alloc::alloc::Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<$t>()).unwrap()
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
            pub fn as_array(&self) -> [$t; 3] {
                use std::convert::TryInto;
                self.as_slice().try_into().unwrap()
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

        impl From<[$t; 3]> for $n {
            #[inline]
            fn from(comps: [$t; 3]) -> Self {
                Self::new(comps[0], comps[1], comps[2])
            }
        }

        impl From<$n> for [$t; 3] {
            #[inline]
            fn from(v: $n) -> Self {
                [v.x, v.y, v.z]
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

macro_rules! ivec4s {
    ($($n:ident, $v2t:ident, $v3t:ident => $t:ident),+) => {
        /// A set of four coordinates which may be interpreted as a point or vector in 4d space,
        /// or as a homogeneous 3d vector or point.
        ///
        /// Generally this distinction between a point and vector is more of a pain than it is worth
        /// to distinguish on a type level, however when converting to and from homogeneous
        /// coordinates it is quite important.
        $(#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
        #[repr(C)]
        pub struct $n {
            pub x: $t,
            pub y: $t,
            pub z: $t,
            pub w: $t,
        }

        impl $n {
            #[inline]
            pub const fn new(x: $t, y: $t, z: $t, w: $t) -> Self {
                $n { x, y, z, w }
            }

            #[inline]
            pub fn broadcast(val: $t) -> Self {
                Self::new(val, val, val, val)
            }

            #[inline]
            pub fn unit_x() -> Self {
                $n{ x: 1, y: 0, z: 0, w: 0 }
            }

            #[inline]
            pub fn unit_y() -> Self {
                $n{ x: 0, y: 1, z: 0, w: 0 }
            }

            #[inline]
            pub fn unit_z() -> Self {
                $n{ x: 0, y: 0, z: 1, w: 0 }
            }

            #[inline]
            pub fn unit_w() -> Self {
                $n{ x: 0, y: 0, z: 0, w: 1 }
            }

            #[inline]
            pub fn dot(&self, other: $n) -> $t {
                (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
            }

            #[inline]
            pub fn reflect(&mut self, normal: $n) {
                *self -= 2 * self.dot(normal) * normal;
            }

            #[inline]
            pub fn reflected(&self, normal: $n) -> Self {
                let mut a = *self;
                a.reflect(normal);
                a
            }

            #[inline]
            pub fn mag(&self) -> $t {
                (self.mag_sq() as f64).sqrt() as $t
            }

            #[inline]
            pub fn mag_sq(&self) -> $t {
                (self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)
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
                Self::broadcast(0 as $t)
            }

            #[inline]
            pub fn one() -> Self {
                Self::broadcast(1 as $t)
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
            pub fn as_slice(&self) -> &[$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const $t, 4)
                }
            }

            #[inline]
            pub fn as_array(&self) -> [$t; 4] {
                use std::convert::TryInto;
                self.as_slice().try_into().unwrap()
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

        impl From<[$t; 4]> for $n {
            #[inline]
            fn from(comps: [$t; 4]) -> Self {
                Self::new(comps[0], comps[1], comps[2], comps[3])
            }
        }

        impl From<$n> for [$t; 4] {
            #[inline]
            fn from(v: $n) -> Self {
                [v.x, v.y, v.z, v.w]
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

        impl std::iter::Sum<$n> for $n {
            fn sum<I>(iter: I) -> Self where I: Iterator<Item = Self> {
                iter.fold($n::zero(), Add::add)
            }
        }
        )+
    }
}

impl Neg for IVec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Neg for IVec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for IVec4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl From<UVec3> for UVec2 {
    #[inline]
    fn from(vec: UVec3) -> Self {
        Self { x: vec.x, y: vec.y }
    }
}

impl From<UVec3> for UVec4 {
    #[inline]
    fn from(vec: UVec3) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
            w: 0,
        }
    }
}

impl From<UVec4> for UVec3 {
    #[inline]
    fn from(vec: UVec4) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }
}

impl From<IVec3> for IVec2 {
    #[inline]
    fn from(vec: IVec3) -> Self {
        Self { x: vec.x, y: vec.y }
    }
}

impl From<IVec3> for IVec4 {
    #[inline]
    fn from(vec: IVec3) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
            w: 0,
        }
    }
}

impl From<IVec4> for IVec3 {
    #[inline]
    fn from(vec: IVec4) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }
}

impl TryFrom<UVec3> for IVec3 {
    type Error = <i32 as TryFrom<u32>>::Error;

    fn try_from(rhv: UVec3) -> Result<Self, Self::Error> {
        Ok(Self {
            x: rhv.x.try_into()?,
            y: rhv.y.try_into()?,
            z: rhv.z.try_into()?,
        })
    }
}

impl TryFrom<IVec3> for UVec3 {
    type Error = <u32 as TryFrom<i32>>::Error;

    fn try_from(rhv: IVec3) -> Result<Self, Self::Error> {
        Ok(Self {
            x: rhv.x.try_into()?,
            y: rhv.y.try_into()?,
            z: rhv.z.try_into()?,
        })
    }
}

/// A macro to implement abs for unsigned and signed IVecs
/// the attributes (x, y, z etc.) must be given in the correct order.
/// example macro use:
/// impl_abs!(IVec2 => [x, y]);
/// or for unsigned
/// impl_abs!(UVec2 => [x, y] nosign);
macro_rules! impl_abs {
    ($n:ident => [$($var:ident),*]) => {
        impl $n {
            #[inline]
            pub fn abs(&self) -> Self {
                Self::new($(self.$var.abs(),)* )
            }
        }
    };
    ($n:ident => [$($var:ident),*] nosign) => {
        impl $n {
            #[inline]
            pub fn abs(&self) -> Self {
                Self::new($(self.$var,)* )
            }
        }
    }
}

ivec2s!((UVec2, UVec3, UVec4) => u32);
ivec2s!((IVec2, IVec3, IVec4) => i32);

ivec3s!((UVec2, UVec3, UVec4) => u32);
ivec3s!((IVec2, IVec3, IVec4) => i32);

ivec4s!(UVec4, UVec2, UVec3 => u32);
ivec4s!(IVec4, IVec2, IVec3 => i32);

impl_abs!(IVec2 => [x, y]);
impl_abs!(IVec3 => [x, y, z]);
impl_abs!(IVec4 => [x, y, z, w]);
impl_abs!(UVec2 => [x, y] nosign);
impl_abs!(UVec3 => [x, y, z] nosign);
impl_abs!(UVec4 => [x, y, z, w] nosign);
