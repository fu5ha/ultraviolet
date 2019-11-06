//! Square matrices.
use std::ops::*;

use crate::vec::*;

macro_rules! mat2s {
    ($($n:ident => $t:ident),+) => {
        /// A 2x2 square matrix.
        ///
        /// Useful for performing linear transformations (rotation, scaling) on 2d vectors.
        $(#[derive(Clone, Copy, Debug)]
        pub struct $n {
            pub cols: [$t; 2],
        }

        impl $n {
            #[inline]
            pub fn new(col1: $t, col2: $t) -> Self {
                $n {
                    cols: [col1, col2],
                }
            }
        }

        impl Mul for $n {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: Self) -> Self {
                let sa = self.cols[0];
                let sb = self.cols[1];
                let oa = rhs.cols[0];
                let ob = rhs.cols[1];
                Self::new(
                    $t::new(
                        sa.x * oa.x + sb.x * oa.y,
                        sa.x * ob.x + sb.x * ob.y,
                    ),
                    $t::new(
                        sa.y * oa.x + sb.y * oa.y,
                        sa.y * ob.x + sb.y * ob.y,
                    ),
                )
            }
        }

        impl Mul<$t> for $n {
            type Output = $t;
            #[inline]
            fn mul(self, rhs: $t) -> $t {
                let a = self.cols[0];
                let b = self.cols[1];
                $t::new(
                    a.x * rhs.x + b.x * rhs.y,
                    a.y * rhs.x + b.y * rhs.y,
                )
            }
        })+
    }
}

mat2s!(Mat2 => Vec2, Wat2 => Wec2);

macro_rules! mat3s {
    ($($n:ident => $t:ident),+) => {
        /// A 3x3 square matrix.
        ///
        /// Useful for performing linear transformations (rotation, scaling) on 3d vectors,
        /// or for performing arbitrary transformations (linear + translation, projection, etc)
        /// on homogeneous 2d vectors
        $(#[derive(Clone, Copy, Debug)]
        pub struct $n {
            pub cols: [$t; 3],
        }

        impl $n {
            #[inline]
            pub fn new(col1: $t, col2: $t, col3: $t) -> Self {
                $n {
                    cols: [col1, col2, col3],
                }
            }
        }

        impl Mul for $n {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: Self) -> Self {
                let sa = self.cols[0];
                let sb = self.cols[1];
                let sc = self.cols[2];
                let oa = rhs.cols[0];
                let ob = rhs.cols[1];
                let oc = rhs.cols[2];
                Self::new(
                    $t::new(
                        sa.x * oa.x + sb.x * oa.y + sc.x * oa.z,
                        sa.x * ob.x + sb.x * ob.y + sc.x * ob.z,
                        sa.x * oc.x + sb.x * oc.y + sc.x * oc.z
                    ),
                    $t::new(
                        sa.y * oa.x + sb.y * oa.y + sc.y * oa.z,
                        sa.y * ob.x + sb.y * ob.y + sc.y * ob.z,
                        sa.y * oc.x + sb.y * oc.y + sc.y * oc.z
                    ),
                    $t::new(
                        sa.z * oa.x + sb.z * oa.y + sc.z * oa.z,
                        sa.z * ob.x + sb.z * ob.y + sc.z * ob.z,
                        sa.z * oc.x + sb.z * oc.y + sc.z * oc.z
                    ),
                )
            }
        }

        impl Mul<$t> for $n {
            type Output = $t;
            #[inline]
            fn mul(self, rhs: $t) -> $t {
                let a = self.cols[0];
                let b = self.cols[1];
                let c = self.cols[2];
                $t::new(
                    a.x * rhs.x + b.x * rhs.y + c.x * rhs.z,
                    a.y * rhs.x + b.y * rhs.y + c.y * rhs.z,
                    a.z * rhs.x + b.z * rhs.y + c.z * rhs.z,
                )
            }
        })+
    }
}

mat3s!(Mat3 => Vec3, Wat3 => Wec3);

macro_rules! mat4s {
    ($($n:ident => $t:ident),+) => {
        /// A 4x4 square matrix.
        ///
        /// Useful for performing linear transformations (rotation, scaling) on 4d vectors,
        /// or for performing arbitrary transformations (linear + translation, projection, etc)
        /// on homogeneous 3d vectors
        $(#[derive(Clone, Copy, Debug)]
        pub struct $n {
            pub cols: [$t; 4],
        }

        impl $n {
            #[inline]
            pub fn new(col1: $t, col2: $t, col3: $t, col4: $t) -> Self {
                $n {
                    cols: [col1, col2, col3, col4],
                }
            }
        }

        impl Mul for $n {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: Self) -> Self {
                let sa = self.cols[0];
                let sb = self.cols[1];
                let sc = self.cols[2];
                let sd = self.cols[3];
                let oa = rhs.cols[0];
                let ob = rhs.cols[1];
                let oc = rhs.cols[2];
                let od = rhs.cols[3];
                Self::new(
                    $t::new(
                        sa.x * oa.x + sb.x * oa.y + sc.x * oa.z + sd.x * oa.w,
                        sa.x * ob.x + sb.x * ob.y + sc.x * ob.z + sd.x * ob.w,
                        sa.x * oc.x + sb.x * oc.y + sc.x * oc.z + sd.x * oc.w,
                        sa.x * od.x + sb.x * od.y + sc.x * od.z + sd.x * od.w,
                    ),
                    $t::new(
                        sa.y * oa.x + sb.y * oa.y + sc.y * oa.z + sd.y * oa.w,
                        sa.y * ob.x + sb.y * ob.y + sc.y * ob.z + sd.y * ob.w,
                        sa.y * oc.x + sb.y * oc.y + sc.y * oc.z + sd.y * oc.w,
                        sa.y * od.x + sb.y * od.y + sc.y * od.z + sd.y * od.w,
                    ),
                    $t::new(
                        sa.z * oa.x + sb.z * oa.y + sc.z * oa.z + sd.z * oa.w,
                        sa.z * ob.x + sb.z * ob.y + sc.z * ob.z + sd.z * ob.w,
                        sa.z * oc.x + sb.z * oc.y + sc.z * oc.z + sd.z * oc.w,
                        sa.z * od.x + sb.z * od.y + sc.z * od.z + sd.z * od.w,
                    ),
                    $t::new(
                        sa.w * oa.x + sb.w * oa.y + sc.w * oa.z + sd.w * oa.w,
                        sa.w * ob.x + sb.w * ob.y + sc.w * ob.z + sd.w * oa.w,
                        sa.w * oc.x + sb.w * oc.y + sc.w * oc.z + sd.w * oc.w,
                        sa.w * od.x + sb.w * od.y + sc.w * od.z + sd.w * od.w,
                    ),
                )
            }
        }

        impl Mul<$t> for $n {
            type Output = $t;
            #[inline]
            fn mul(self, rhs: $t) -> $t {
                let a = self.cols[0];
                let b = self.cols[1];
                let c = self.cols[2];
                let d = self.cols[3];
                $t::new(
                    a.x * rhs.x + b.x * rhs.y + c.x * rhs.z + d.x * rhs.w,
                    a.y * rhs.x + b.y * rhs.y + c.y * rhs.z + d.y * rhs.w,
                    a.z * rhs.x + b.z * rhs.y + c.z * rhs.z + d.z * rhs.w,
                    a.w * rhs.x + b.w * rhs.y + c.w * rhs.z + d.z * rhs.w,
                )
            }
        })+
    }
}

mat4s!(Mat4 => Vec4, Wat4 => Wec4);
