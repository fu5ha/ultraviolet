use std::ops::*;

use crate::util::*;
use crate::*;

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
                $n{ x: $t::splat(1.0), y: $t::splat(0.0), z: $t::splat(0.0), w: $t::splat(0.0) }
            }

            #[inline]
            pub fn unit_y() -> Self {
                $n{ x: $t::splat(0.0), y: $t::splat(1.0), z: $t::splat(0.0), w: $t::splat(0.0) }
            }

            #[inline]
            pub fn unit_z() -> Self {
                $n{ x: $t::splat(0.0), y: $t::splat(0.0), z: $t::splat(1.0), w: $t::splat(0.0) }
            }

            #[inline]
            pub fn unit_w() -> Self {
                $n{ x: $t::splat(0.0), y: $t::splat(0.0), z: $t::splat(0.0), w: $t::splat(1.0) }
            }

            #[inline]
            pub fn dot(&self, other: $n) -> $t {
                self.x.mul_add(other.x, self.y.mul_add(other.y, self.z.mul_add(other.z, self.w * other.w)))
            }

            #[inline]
            pub fn reflect(&mut self, normal: $n) {
                *self -= $t::splat(2.0) * self.dot(normal) * normal;
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
                Self::broadcast($t::splat(0.0))
            }

            #[inline]
            pub fn one() -> Self {
                Self::broadcast($t::splat(1.0))
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
                self * $t::splat(-1.0)
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

vec4s!(
    Vec4, Vec2, Vec3 => f32,
    Vec4x4, Vec2x4, Vec3x4 => f32x4,
    Vec4x8, Vec2x8, Vec3x8 => f32x8,

    DVec4, DVec2, DVec3 => f64,
    DVec4x2, DVec2x2, DVec3x2 => f64x2,
    DVec4x4, DVec2x4, DVec3x4 => f64x4
);

// SCALAR VEC4 IMPLS

macro_rules! impl_scalar_vec4s {
    ($(($vt:ident, $v3t:ident) => $t:ident),+) => {
        $(impl $vt {
            #[inline]
            pub fn refract(&mut self, normal: Self, eta: $t) {
                *self = self.refracted(normal, eta);
            }

            #[inline]
            pub fn refracted(&self, normal: Self, eta: $t) -> Self {
                let n = normal;
                let i = *self;
                let ndi = n.dot(i);
                let k = 1.0 - eta * eta * (1.0 - ndi * ndi);
                if k < 0.0 {
                    Self::zero()
                } else {
                    i * eta - (eta * ndi + k.sqrt()) * n
                }
            }
        }

        impl From<$v3t> for $vt {
            #[inline]
            fn from(vec: $v3t) -> Self {
                Self {
                    x: vec.x,
                    y: vec.y,
                    z: vec.z,
                    w: 0.0,
                }
            }
        }

        impl PartialEq for $vt {
            fn eq(&self, other: &Self) -> bool {
                self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
            }

            fn ne(&self, other: &Self) -> bool {
                self.x != other.x || self.y != other.y || self.z != other.z || self.w != other.w
            }
        })+
    }
}

impl_scalar_vec4s!(
    (Vec4, Vec3) => f32,
    (DVec4, DVec3) => f64
);

// WIDE VEC4 IMPLS

macro_rules! impl_wide_vec4s {
    ($($vt:ident => $tt:ident, $t:ident, $maskt:ident, $nonwidet:ident, $v3t:ident),+) => {
        $(impl $vt {
            #[inline]
            pub fn new_splat(x: $tt, y: $tt, z: $tt, w: $tt) -> Self {
                Self {
                    x: $t::splat(x),
                    y: $t::splat(y),
                    z: $t::splat(z),
                    w: $t::splat(w),
                }
            }

            #[inline]
            pub fn splat(vec: $nonwidet) -> Self {
                Self {
                    x: $t::splat(vec.x),
                    y: $t::splat(vec.y),
                    z: $t::splat(vec.z),
                    w: $t::splat(vec.w),
                }
            }

            /// Blend two vectors together lanewise using `mask` as a mask.
            ///
            /// This is essentially a bitwise blend operation, such that any point where
            /// there is a 1 bit in `mask`, the output will put the bit from `tru`, while
            /// where there is a 0 bit in `mask`, the output will put the bit from `fals`
            #[inline]
            pub fn blend(mask: $maskt, tru: Self, fals: Self) -> Self {
                Self {
                    x: mask.blend(tru.x, fals.x),
                    y: mask.blend(tru.y, fals.y),
                    z: mask.blend(tru.z, fals.z),
                    w: mask.blend(tru.w, fals.w),
                }
            }
        }

        impl From<$nonwidet> for $vt {
            #[inline]
            fn from(vec: $nonwidet) -> Self {
                Self::splat(vec)
            }
        }

        impl From<$v3t> for $vt {
            #[inline]
            fn from(vec: $v3t) -> Self {
                Self {
                    x: vec.x,
                    y: vec.y,
                    z: vec.z,
                    w: $t::splat(0.0),
                }
            }
        })+
    };
}

impl_wide_vec4s!(
    Vec4x4 => f32, f32x4, m32x4, Vec4, Vec3x4,
    Vec4x8 => f32, f32x8, m32x8, Vec4, Vec3x8,

    DVec4x2 => f64, f64x2, m64x2, DVec4, DVec3x2,
    DVec4x4 => f64, f64x4, m64x4, DVec4, DVec3x4
);

impl Into<[Vec4; 4]> for Vec4x4 {
    #[inline]
    fn into(self) -> [Vec4; 4] {
        let xs: [f32; 4] = self.x.into();
        let ys: [f32; 4] = self.y.into();
        let zs: [f32; 4] = self.z.into();
        let ws: [f32; 4] = self.w.into();
        [
            Vec4::new(xs[0], ys[0], zs[0], ws[0]),
            Vec4::new(xs[1], ys[1], zs[1], ws[1]),
            Vec4::new(xs[2], ys[2], zs[2], ws[2]),
            Vec4::new(xs[3], ys[3], zs[3], ws[3]),
        ]
    }
}

impl From<[Vec4; 4]> for Vec4x4 {
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

impl Into<[Vec4; 8]> for Vec4x8 {
    #[inline]
    fn into(self) -> [Vec4; 8] {
        let xs: [f32; 8] = self.x.into();
        let ys: [f32; 8] = self.y.into();
        let zs: [f32; 8] = self.z.into();
        let ws: [f32; 8] = self.z.into();
        [
            Vec4::new(xs[0], ys[0], zs[0], ws[0]),
            Vec4::new(xs[1], ys[1], zs[1], ws[1]),
            Vec4::new(xs[2], ys[2], zs[2], ws[2]),
            Vec4::new(xs[3], ys[3], zs[3], ws[3]),
            Vec4::new(xs[4], ys[4], zs[4], ws[4]),
            Vec4::new(xs[5], ys[5], zs[5], ws[5]),
            Vec4::new(xs[6], ys[6], zs[6], ws[6]),
            Vec4::new(xs[7], ys[7], zs[7], ws[7]),
        ]
    }
}

impl From<[Vec4; 8]> for Vec4x8 {
    #[inline]
    fn from(vecs: [Vec4; 8]) -> Self {
        Self {
            x: f32x8::from([
                vecs[0].x, vecs[1].x, vecs[2].x, vecs[3].x, vecs[4].x, vecs[5].x, vecs[6].x,
                vecs[7].x,
            ]),
            y: f32x8::from([
                vecs[0].y, vecs[1].y, vecs[2].y, vecs[3].y, vecs[4].y, vecs[5].y, vecs[6].y,
                vecs[7].y,
            ]),
            z: f32x8::from([
                vecs[0].z, vecs[1].z, vecs[2].z, vecs[3].z, vecs[4].z, vecs[5].z, vecs[6].z,
                vecs[7].z,
            ]),
            w: f32x8::from([
                vecs[0].w, vecs[1].w, vecs[2].w, vecs[3].w, vecs[4].w, vecs[5].w, vecs[6].w,
                vecs[7].w,
            ]),
        }
    }
}

impl Into<[DVec4; 2]> for DVec4x2 {
    #[inline]
    fn into(self) -> [DVec4; 2] {
        let xs: [f64; 2] = self.x.into();
        let ys: [f64; 2] = self.y.into();
        let zs: [f64; 2] = self.z.into();
        let ws: [f64; 2] = self.w.into();
        [
            DVec4::new(xs[0], ys[0], zs[0], ws[0]),
            DVec4::new(xs[1], ys[1], zs[1], ws[1]),
        ]
    }
}

impl From<[DVec4; 2]> for DVec4x2 {
    #[inline]
    fn from(vecs: [DVec4; 2]) -> Self {
        Self {
            x: f64x2::from([vecs[0].x, vecs[1].x]),
            y: f64x2::from([vecs[0].y, vecs[1].y]),
            z: f64x2::from([vecs[0].z, vecs[1].z]),
            w: f64x2::from([vecs[0].w, vecs[1].w]),
        }
    }
}

impl Into<[DVec4; 4]> for DVec4x4 {
    #[inline]
    fn into(self) -> [DVec4; 4] {
        let xs: [f64; 4] = self.x.into();
        let ys: [f64; 4] = self.y.into();
        let zs: [f64; 4] = self.z.into();
        let ws: [f64; 4] = self.w.into();
        [
            DVec4::new(xs[0], ys[0], zs[0], ws[0]),
            DVec4::new(xs[1], ys[1], zs[1], ws[1]),
            DVec4::new(xs[2], ys[2], zs[2], ws[2]),
            DVec4::new(xs[3], ys[3], zs[3], ws[3]),
        ]
    }
}

impl From<[DVec4; 4]> for DVec4x4 {
    #[inline]
    fn from(vecs: [DVec4; 4]) -> Self {
        Self {
            x: f64x4::from([vecs[0].x, vecs[1].x, vecs[2].x, vecs[3].x]),
            y: f64x4::from([vecs[0].y, vecs[1].y, vecs[2].y, vecs[3].y]),
            z: f64x4::from([vecs[0].z, vecs[1].z, vecs[2].z, vecs[3].z]),
            w: f64x4::from([vecs[0].w, vecs[1].w, vecs[2].w, vecs[3].w]),
        }
    }
}
