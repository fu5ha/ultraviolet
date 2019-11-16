//! Square matrices.
use std::ops::*;

use crate::vec::*;

use wide::f32x4;

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

            #[inline]
            pub fn layout() -> alloc::alloc::Layout {
                alloc::alloc::Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<$t>()).unwrap()
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
    ($($n:ident => $vt:ident, $t:ident),+) => {
        /// A 3x3 square matrix.
        ///
        /// Useful for performing linear transformations (rotation, scaling) on 3d vectors,
        /// or for performing arbitrary transformations (linear + translation, projection, etc)
        /// on homogeneous 2d vectors
        $(#[derive(Clone, Copy, Debug)]
        pub struct $n {
            pub cols: [$vt; 3],
        }

        impl $n {
            #[inline]
            pub fn new(col1: $vt, col2: $vt, col3: $vt) -> Self {
                $n {
                    cols: [col1, col2, col3],
                }
            }

            #[inline]
            pub fn identity() -> Self {
                Self::new(
                    $vt::new($t::from(1.0), $t::from(0.0), $t::from(0.0)),
                    $vt::new($t::from(0.0), $t::from(1.0), $t::from(0.0)),
                    $vt::new($t::from(0.0), $t::from(0.0), $t::from(1.0)))
            }

            /// Angles are applied in the order roll -> pitch -> yaw
            ///
            /// - Yaw is rotation inside the xz plane ("around the y axis")
            /// - Pitch is rotation inside the yz plane ("around the x axis")
            /// - Roll is rotation inside the xy plane ("around the z axis")
            #[inline]
            pub fn from_euler_angles(roll: $t, pitch: $t, yaw: $t) -> Self {
                let (sr, cr) = roll.sin_cos();
                let (sp, cp) = pitch.sin_cos();
                let (sy, cy) = yaw.sin_cos();

                Self::new(
                    $vt::new(cy * cp, cy * sp * sr - sy * cr, cy * sp * cr + sy * sr),
                    $vt::new(sy * cp, sy * sp * sr + cy * cr, sy * sp * cr - cy * sr),
                    $vt::new(-sp, cp * sr, cp * cr))
            }

            #[inline]
            pub fn layout() -> alloc::alloc::Layout {
                alloc::alloc::Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<$t>()).unwrap()
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
                    $vt::new(
                        sa.x * oa.x + sb.x * oa.y + sc.x * oa.z,
                        sa.x * ob.x + sb.x * ob.y + sc.x * ob.z,
                        sa.x * oc.x + sb.x * oc.y + sc.x * oc.z
                    ),
                    $vt::new(
                        sa.y * oa.x + sb.y * oa.y + sc.y * oa.z,
                        sa.y * ob.x + sb.y * ob.y + sc.y * ob.z,
                        sa.y * oc.x + sb.y * oc.y + sc.y * oc.z
                    ),
                    $vt::new(
                        sa.z * oa.x + sb.z * oa.y + sc.z * oa.z,
                        sa.z * ob.x + sb.z * ob.y + sc.z * ob.z,
                        sa.z * oc.x + sb.z * oc.y + sc.z * oc.z
                    ),
                )
            }
        }

        impl Mul<$vt> for $n {
            type Output = $vt;
            #[inline]
            fn mul(self, rhs: $vt) -> $vt {
                let a = self.cols[0];
                let b = self.cols[1];
                let c = self.cols[2];
                $vt::new(
                    a.x * rhs.x + b.x * rhs.y + c.x * rhs.z,
                    a.y * rhs.x + b.y * rhs.y + c.y * rhs.z,
                    a.z * rhs.x + b.z * rhs.y + c.z * rhs.z,
                )
            }
        })+
    }
}

mat3s!(Mat3 => Vec3, f32, Wat3 => Wec3, f32x4);

macro_rules! mat4s {
    ($($n:ident => $vt:ident, $v3t:ident, $t:ident),+) => {
        /// A 4x4 square matrix.
        ///
        /// Useful for performing linear transformations (rotation, scaling) on 4d vectors,
        /// or for performing arbitrary transformations (linear + translation, projection, etc)
        /// on homogeneous 3d vectors
        $(#[derive(Clone, Copy, Debug)]
        pub struct $n {
            pub cols: [$vt; 4],
        }

        impl $n {
            #[inline]
            pub fn new(col1: $vt, col2: $vt, col3: $vt, col4: $vt) -> Self {
                $n {
                    cols: [col1, col2, col3, col4],
                }
            }

            #[inline]
            pub fn identity() -> Self {
                Self::new(
                    $vt::new($t::from(1.0), $t::from(0.0), $t::from(0.0), $t::from(0.0)),
                    $vt::new($t::from(0.0), $t::from(1.0), $t::from(0.0), $t::from(0.0)),
                    $vt::new($t::from(0.0), $t::from(0.0), $t::from(1.0), $t::from(0.0)),
                    $vt::new($t::from(0.0), $t::from(0.0), $t::from(0.0), $t::from(1.0)))
            }

            #[inline]
            pub fn from_translation(trans: $v3t) -> Self {
                Self::new(
                    $vt::new($t::from(1.0), $t::from(0.0), $t::from(0.0), trans.x),
                    $vt::new($t::from(0.0), $t::from(1.0), $t::from(0.0), trans.y),
                    $vt::new($t::from(0.0), $t::from(0.0), $t::from(1.0), trans.z),
                    $vt::new($t::from(0.0), $t::from(0.0), $t::from(0.0), $t::from(1.0)))
            }

            /// Angles are applied in the order roll -> pitch -> yaw
            ///
            /// - Yaw is rotation inside the xz plane ("around the y axis")
            /// - Pitch is rotation inside the yz plane ("around the x axis")
            /// - Roll is rotation inside the xy plane ("around the z axis")
            #[inline]
            pub fn from_euler_angles(roll: $t, pitch: $t, yaw: $t) -> Self {
                let (sr, cr) = roll.sin_cos();
                let (sp, cp) = pitch.sin_cos();
                let (sy, cy) = yaw.sin_cos();

                Self::new(
                    $vt::new(cy * cp, cy * sp * sr - sy * cr, cy * sp * cr + sy * sr, $t::from(0.0)),
                    $vt::new(sy * cp, sy * sp * sr + cy * cr, sy * sp * cr - cy * sr, $t::from(0.0)),
                    $vt::new(-sp, cp * sr, cp * cr, $t::from(0.0)),
                    $vt::new($t::from(0.0), $t::from(0.0), $t::from(0.0), $t::from(1.0)))
            }

            #[inline]
            pub fn layout() -> alloc::alloc::Layout {
                alloc::alloc::Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<$t>()).unwrap()
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
                    $vt::new(
                        sa.x * oa.x + sb.x * oa.y + sc.x * oa.z + sd.x * oa.w,
                        sa.x * ob.x + sb.x * ob.y + sc.x * ob.z + sd.x * ob.w,
                        sa.x * oc.x + sb.x * oc.y + sc.x * oc.z + sd.x * oc.w,
                        sa.x * od.x + sb.x * od.y + sc.x * od.z + sd.x * od.w,
                    ),
                    $vt::new(
                        sa.y * oa.x + sb.y * oa.y + sc.y * oa.z + sd.y * oa.w,
                        sa.y * ob.x + sb.y * ob.y + sc.y * ob.z + sd.y * ob.w,
                        sa.y * oc.x + sb.y * oc.y + sc.y * oc.z + sd.y * oc.w,
                        sa.y * od.x + sb.y * od.y + sc.y * od.z + sd.y * od.w,
                    ),
                    $vt::new(
                        sa.z * oa.x + sb.z * oa.y + sc.z * oa.z + sd.z * oa.w,
                        sa.z * ob.x + sb.z * ob.y + sc.z * ob.z + sd.z * ob.w,
                        sa.z * oc.x + sb.z * oc.y + sc.z * oc.z + sd.z * oc.w,
                        sa.z * od.x + sb.z * od.y + sc.z * od.z + sd.z * od.w,
                    ),
                    $vt::new(
                        sa.w * oa.x + sb.w * oa.y + sc.w * oa.z + sd.w * oa.w,
                        sa.w * ob.x + sb.w * ob.y + sc.w * ob.z + sd.w * oa.w,
                        sa.w * oc.x + sb.w * oc.y + sc.w * oc.z + sd.w * oc.w,
                        sa.w * od.x + sb.w * od.y + sc.w * od.z + sd.w * od.w,
                    ),
                )
            }
        }

        impl Mul<$vt> for $n {
            type Output = $vt;
            #[inline]
            fn mul(self, rhs: $vt) -> $vt {
                let a = self.cols[0];
                let b = self.cols[1];
                let c = self.cols[2];
                let d = self.cols[3];
                $vt::new(
                    a.x * rhs.x + b.x * rhs.y + c.x * rhs.z + d.x * rhs.w,
                    a.y * rhs.x + b.y * rhs.y + c.y * rhs.z + d.y * rhs.w,
                    a.z * rhs.x + b.z * rhs.y + c.z * rhs.z + d.z * rhs.w,
                    a.w * rhs.x + b.w * rhs.y + c.w * rhs.z + d.z * rhs.w,
                )
            }
        })+
    }
}

mat4s!(Mat4 => Vec4, Vec3, f32, Wat4 => Wec4, Wec3, f32x4);
