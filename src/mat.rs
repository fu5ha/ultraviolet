//! Square matrices.
use std::ops::*;

use crate::*;

macro_rules! mat2s {
    ($($n:ident => $m3t:ident, $v3t:ident, $vt:ident, $t:ident),+) => {
        $(/// A 2x2 square matrix.
        ///
        /// Useful for performing linear transformations (rotation, scaling) on 2d vectors.
        #[derive(Clone, Copy, Debug, PartialEq)]
        #[repr(C)]
        pub struct $n {
            pub cols: [$vt; 2],
        }

        derive_default_identity!($n);

        impl $n {
            #[inline]
            pub const fn new(col1: $vt, col2: $vt) -> Self {
                $n {
                    cols: [col1, col2],
                }
            }

            #[inline]
            pub fn identity() -> Self {
                Self::new(
                    $vt::new($t::splat(1.0), $t::splat(0.0)),
                    $vt::new($t::splat(0.0), $t::splat(1.0)),
                )
            }

            /// Turn this into a homogeneous 2d transformation matrix.
            #[inline]
            pub fn into_homogeneous(self) -> $m3t {
                $m3t::new(
                    self.cols[0].into(),
                    self.cols[1].into(),
                    $v3t::new($t::splat(0.0), $t::splat(0.0), $t::splat(1.0))
                )
            }

            #[inline]
            pub fn transpose(&mut self) {
                *self = self.transposed();
            }

            #[inline]
            pub fn transposed(&self) -> Self {
                let (x0, y0) = self.cols[0].into();
                let (x1, y1) = self.cols[1].into();
                Self::new(
                    $vt::new(x0, x1),
                    $vt::new(y0, y1),
                )
            }

            #[inline]
            pub fn determinant(&self) -> $t {
                (self.cols[0].x * self.cols[1].y) - (self.cols[1].x * self.cols[0].y)
            }

            /// The adjugate of this matrix, i.e. the transpose of
            /// the cofactor matrix.
            ///
            /// This is equivalent to the inverse
            /// but without dividing by the determinant of the matrix,
            /// which can be useful in some contexts for better performance.
            ///
            /// One such case is when this matrix is interpreted as a
            /// a homogeneous transformation matrix, in which case uniform scaling will
            /// not affect the resulting projected 3d version of transformed points or
            /// vectors.
            #[inline]
            pub fn adjugate(&self) -> Self {
                Self::new(
                    $vt::new(self.cols[1].y, -self.cols[0].y),
                    $vt::new(-self.cols[1].x, self.cols[0].x),
                )
            }

            /// If this matrix is not currently invertable, this function will return
            /// an invalid inverse. This status is not checked by the library.
            #[inline]
            pub fn inverse(&mut self) {
                let n = self.inversed();
                *self = n;
            }

            /// If this matrix is not currently invertable, this function will return
            /// an invalid inverse. This status is not checked by the library.
            #[inline]
            pub fn inversed(&self) -> Self {
                let det = self.determinant();
                let inv_det = $t::splat(1.0) / det;

                inv_det * self.adjugate()
            }

            /// Get the [`core::alloc::Layout`] of `Self`
            #[inline]
            pub fn layout() -> alloc::alloc::Layout {
                alloc::alloc::Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<$vt>()).unwrap()
            }

            /// Interpret `self` as a statically-sized array of its base numeric type
            #[inline]
            pub fn as_array(&self) -> &[$t; 4] {
                let ptr = self as *const $n as *const [$t; 4];
                unsafe { &*ptr }
            }

            /// Interpret `self` as a statically-sized array of its base numeric type
            #[inline]
            pub fn as_mut_array(&mut self) -> &mut [$t; 4] {
                let ptr = self as *mut $n as *mut [$t; 4];
                unsafe { &mut *ptr }
            }

            /// Interpret `self` as a statically-sized array of its component (column) vector type
            #[inline]
            pub fn as_component_array(&self) -> &[$vt; 2] {
                let ptr = self as *const $n as *const [$vt; 2];
                unsafe { &*ptr }
            }

            /// Interpret `self` as a statically-sized array of its component (column) vector type
            #[inline]
            pub fn as_mut_component_array(&mut self) -> &mut [$vt; 2] {
                let ptr = self as *mut $n as *mut [$vt; 2];
                unsafe { &mut *ptr }
            }

            /// Interpret `self` as a slice of its base numeric type
            #[inline]
            pub fn as_slice(&self) -> &[$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const $t, 4)
                }
            }

            /// Interpret `self` as a slice of its base numeric type
            #[inline]
            pub fn as_mut_slice(&mut self) -> &mut [$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(self as *mut $n as *mut $t, 4)
                }
            }

            /// Interpret `self` as a slice of its component (column) vector type
            #[inline]
            pub fn as_component_slice(&self) -> &[$vt] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const $vt, 2)
                }
            }

            /// Interpret `self` as a slice of its component (column) vector type
            #[inline]
            pub fn as_mut_component_slice(&mut self) -> &mut [$vt] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(self as *mut $n as *mut $vt, 2)
                }
            }

            /// Interpret `self` as a slice of bytes
            #[inline]
            pub fn as_byte_slice(&self) -> &[u8] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const u8, 4 * std::mem::size_of::<$t>())
                }
            }

            /// Interpret `self` as a slice of bytes
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
            pub const fn as_ptr(&self) -> *const $t {
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

        impl Mul for $n {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: Self) -> Self {
                let sa = self.cols[0];
                let sb = self.cols[1];
                let oa = rhs.cols[0];
                let ob = rhs.cols[1];
                Self::new(
                    $vt::new(
                        (sa.x * oa.x) + (sb.x * oa.y),
                        (sa.y * oa.x) + (sb.y * oa.y),
                    ),
                    $vt::new(
                        (sa.x * ob.x) + (sb.x * ob.y),
                        (sa.y * ob.x) + (sb.y * ob.y),
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
                $vt::new(
                    (a.x * rhs.x) + (b.x * rhs.y),
                    (a.y * rhs.x) + (b.y * rhs.y),
                )
            }
        }

        impl Mul<$t> for $n {
            type Output = $n;
            #[inline]
            fn mul(self, rhs: $t) -> $n {
                $n::new(
                    self.cols[0] * rhs,
                    self.cols[1] * rhs,
                )
            }
        }

        impl Mul<$n> for $t {
            type Output = $n;
            #[inline]
            fn mul(self, rhs: $n) -> $n {
                $n::new(
                    rhs.cols[0] * self,
                    rhs.cols[1] * self,
                )
            }
        }

        impl Add for $n {
            type Output = Self;
            #[inline]
            fn add(self, rhs: $n) -> Self {
                $n::new(
                    self.cols[0] + rhs.cols[0],
                    self.cols[1] + rhs.cols[1],
                )
            }
        }

        impl AddAssign for $n {
            #[inline]
            fn add_assign(&mut self, rhs: $n) {
                *self = *self + rhs;
            }
        }

        impl From<[$t; 4]> for $n {
            #[inline]
            fn from(comps: [$t; 4]) -> Self {
                Self::new(
                    $vt::new(comps[0], comps[1]),
                    $vt::new(comps[2], comps[3])
                )
            }
        }

        impl From<[[$t; 2]; 2]> for $n {
            #[inline]
            fn from(comps: [[$t; 2]; 2]) -> Self {
                Self::new(
                    $vt::new(comps[0][0], comps[0][1]),
                    $vt::new(comps[1][0], comps[1][1])
                )
            }
        }

        impl From<$n> for [[$t; 2]; 2] {
            #[inline]
            fn from(mat2: $n) -> Self {
                [
                    [mat2.cols[0].x, mat2.cols[0].y],
                    [mat2.cols[1].x, mat2.cols[1].y],
                ]
            }
        }

        impl From<&[$t; 4]> for $n {
            #[inline]
            fn from(comps: &[$t; 4]) -> Self {
                Self::from(*comps)
            }
        }

        impl Index<usize> for $n {
            type Output = $vt;

            fn index(&self, index: usize) -> &Self::Output {
                &self.cols[index]
            }
        }

        impl IndexMut<usize> for $n {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.cols[index]
            }
        }
        )+
    }
}

mat2s!(
    Mat2 => Mat3, Vec3, Vec2, f32,
    Mat2x4 => Mat3x4, Vec3x4, Vec2x4, f32x4,
    Mat2x8 => Mat3x8, Vec3x8, Vec2x8, f32x8
);

#[cfg(feature = "f64")]
mat2s!(
    DMat2 => DMat3, DVec3, DVec2, f64,
    DMat2x2 => DMat3x2, DVec3x2, DVec2x2, f64x2,
    DMat2x4 => DMat3x4, DVec3x4, DVec2x4, f64x4
);

macro_rules! mat3s {
    ($($n:ident => $rt:ident, $bt:ident, $m4t:ident, $v4t:ident, $v2t:ident, $vt:ident, $t:ident),+) => {
        $(/// A 3x3 square matrix.
        ///
        /// Useful for performing linear transformations (rotation, scaling) on 3d vectors,
        /// or for performing arbitrary transformations (linear +   translation, projection, etc)
        /// on homogeneous 2d vectors
        #[derive(Clone, Copy, Debug, PartialEq)]
        #[repr(C)]
        pub struct $n {
            pub cols: [$vt; 3],
        }

        derive_default_identity!($n);

        impl $n {
            #[inline]
            pub const fn new(col1: $vt, col2: $vt, col3: $vt) -> Self {
                $n {
                    cols: [col1, col2, col3],
                }
            }

            /// Assumes homogeneous 2d coordinates.
            #[inline]
            pub fn from_translation(trans: $v2t) -> Self {
                Self::new(
                    $vt::new($t::splat(1.0), $t::splat(0.0), $t::splat(0.0)),
                    $vt::new($t::splat(0.0), $t::splat(1.0), $t::splat(0.0)),
                    $vt::new(trans.x, trans.y, $t::splat(1.0)))
            }

            /// Assumes homogeneous 2d coordinates.
            #[inline]
            pub fn from_scale_homogeneous(scale: $t) -> Self {
                let zero = $t::splat(0.0);
                Self::new(
                    $vt::new(scale, zero, zero),
                    $vt::new(zero, scale, zero),
                    $vt::new(zero, zero, $t::splat(1.0)),
                )
            }

            /// Assumes homogeneous 2d coordinates.
            #[inline]
            pub fn from_nonuniform_scale_homogeneous(scale: $v2t) -> Self {
                let zero = $t::splat(0.0);
                Self::new(
                    $vt::new(scale.x, zero, zero),
                    $vt::new(zero, scale.y, zero),
                    $vt::new(zero, zero, $t::splat(1.0)),
                )
            }

            /// Builds a homogeneous 2d rotation matrix (in the xy plane) from a given angle in radians.
            #[inline]
            pub fn from_rotation_homogeneous(angle: $t) -> Self {
                let (s, c) = angle.sin_cos();
                let zero = $t::splat(0.0);
                Self::new(
                    $vt::new(c, s, zero),
                    $vt::new(-s, c, zero),
                    $vt::new(zero, zero, $t::splat(1.0)),
                )
            }

            #[inline]
            pub fn from_scale(scale: $t) -> Self {
                let zero = $t::splat(0.0);
                Self::new(
                    $vt::new(scale, zero, zero),
                    $vt::new(zero, scale, zero),
                    $vt::new(zero, zero, scale),
                )
            }

            #[inline]
            pub fn from_nonuniform_scale(scale: $vt) -> Self {
                let zero = $t::splat(0.0);
                Self::new(
                    $vt::new(scale.x, zero, zero),
                    $vt::new(zero, scale.y, zero),
                    $vt::new(zero, zero, scale.z),
                )
            }

            #[inline]
            pub fn identity() -> Self {
                Self::new(
                    $vt::new($t::splat(1.0), $t::splat(0.0), $t::splat(0.0)),
                    $vt::new($t::splat(0.0), $t::splat(1.0), $t::splat(0.0)),
                    $vt::new($t::splat(0.0), $t::splat(0.0), $t::splat(1.0)))
            }

            /// Angles are applied in the order roll -> pitch -> yaw.
            ///
            /// - Yaw is rotation inside the xz plane ("around the y axis")
            /// - Pitch is rotation inside the yz plane ("around the x axis")
            /// - Roll is rotation inside the xy plane ("around the z axis")
            #[inline]
            #[allow(unused_variables)]
            pub fn from_euler_angles(roll: $t, pitch: $t, yaw: $t) -> Self {
                let (sin_yaw, cos_yaw) = yaw.sin_cos();
                let (sin_pitch, cos_pitch) = pitch.sin_cos();
                let (sin_roll, cos_roll) = roll.sin_cos();

                let sin_pitch_sin_roll = sin_pitch * sin_roll;
                let sin_pitch_cos_roll = sin_pitch * cos_roll;

                let m00 = cos_yaw * cos_roll + sin_pitch * sin_yaw* sin_roll;
                let m10 = cos_pitch * sin_roll;
                let m20 = -cos_roll * sin_yaw + cos_yaw * sin_pitch * sin_roll;
                let m01 = cos_roll * sin_pitch * sin_yaw - cos_yaw * sin_roll;
                let m11 = cos_pitch * cos_roll;
                let m21 = cos_yaw * cos_roll * sin_pitch + sin_yaw * sin_roll;
                let m02 = cos_pitch * sin_yaw;
                let m12 = -sin_pitch;
                let m22 = cos_pitch * cos_yaw;

                // think transposed as arguments are columns
                Self::new(
                    $vt::new(m00, m10, m20),
                    $vt::new(m01, m11, m21),
                    $vt::new(m02, m12, m22),
                )
            }

            /// Create a new rotation matrix from a rotation "around the x axis". This is
            /// here as a convenience function for users coming from other libraries; it is
            /// more proper to think of this as a rotation *in the yz plane*.
            #[inline]
            pub fn from_rotation_x(angle: $t) -> Self {
                let (sin, cos) = angle.sin_cos();
                let zero = $t::splat(0.0);
                let one = $t::splat(1.0);

                // think transposed as arguments are columns
                Self::new(
                    $vt::new(one, zero, zero),
                    $vt::new(zero, cos, sin),
                    $vt::new(zero, -sin, cos),
                )
            }

            /// Create a new rotation matrix from a rotation "around the y axis". This is
            /// here as a convenience function for users coming from other libraries; it is
            /// more proper to think of this as a rotation *in the xz plane*.
            #[inline]
            pub fn from_rotation_y(angle: $t) -> Self {
                let (sin, cos) = angle.sin_cos();
                let zero = $t::splat(0.0);
                let one = $t::splat(1.0);

                // think transposed as arguments are columns
                Self::new(
                    $vt::new(cos, zero, -sin),
                    $vt::new(zero, one, zero),
                    $vt::new(sin, zero, cos),
                )
            }

            /// Create a new rotation matrix from a rotation "around the z axis". This is
            /// here as a convenience function for users coming from other libraries; it is
            /// more proper to think of this as a rotation *in the xy plane*.
            #[inline]
            pub fn from_rotation_z(angle: $t) -> Self {
                let (sin, cos) = angle.sin_cos();
                let zero = $t::splat(0.0);
                let one = $t::splat(1.0);

                // think transposed as arguments are columns
                Self::new(
                    $vt::new(cos, sin, zero),
                    $vt::new(-sin, cos, zero),
                    $vt::new(zero, zero, one),
                )
            }

            /// Create a new rotation matrix from a rotation around the given axis.
            /// This is here as a convenience function for users coming from other libraries.
            #[inline]
            pub fn from_rotation_around(axis: $vt, angle: $t) -> Self {
                let (sin, cos) = angle.sin_cos();
                let mul = $t::splat(1.0) - cos;

                let x_sin = axis.x * sin;
                let y_sin = axis.y * sin;
                let z_sin = axis.z * sin;

                let xy_mul = axis.x * axis.y * mul;
                let xz_mul = axis.x * axis.z * mul;
                let yz_mul = axis.y * axis.z * mul;

                let m00 = (axis.x * axis.x).mul_add(mul, cos);
                let m10 = xy_mul + z_sin;
                let m20 = xz_mul - y_sin;
                let m01 = xy_mul - z_sin;
                let m11 = (axis.y * axis.y).mul_add(mul, cos);
                let m21 = yz_mul + x_sin;
                let m02 = xz_mul + y_sin;
                let m12 = yz_mul - x_sin;
                let m22 = (axis.z * axis.z).mul_add(mul, cos);

                // think transposed as arguments are columns
                Self::new(
                    $vt::new(m00, m10, m20),
                    $vt::new(m01, m11, m21),
                    $vt::new(m02, m12, m22),
                )
            }

            /// Construct a rotation matrix given a bivector which defines a plane and rotation orientation,
            /// and a rotation angle.
            ///
            /// `plane` must be normalized!
            ///
            /// This is the equivalent of an axis-angle rotation.
            #[inline]
            pub fn from_angle_plane(angle: $t, plane: $bt) -> Self {
                $rt::from_angle_plane(angle, plane).into_matrix()
            }

            #[inline]
            pub fn into_homogeneous(self) -> $m4t {
                let zero = $t::splat(0.0);
                let one = $t::splat(1.0);
                $m4t::new(
                    self.cols[0].into(),
                    self.cols[1].into(),
                    self.cols[2].into(),
                    $v4t::new(zero, zero, zero, one)
                )
            }

            #[inline]
            pub fn determinant(&self) -> $t {
                self.cols[0].x.mul_add(
                    self.cols[1].y.mul_add(self.cols[2].z, -(self.cols[2].y * self.cols[1].z)),
                    -(self.cols[1].x.mul_add(
                        self.cols[0].y.mul_add(self.cols[2].z, -(self.cols[2].y * self.cols[0].z)),
                        -(self.cols[2].x * self.cols[0].y.mul_add(self.cols[1].z, -(self.cols[1].y * self.cols[0].z)))
                    ))
                )
            }

            /// The adjugate of this matrix, i.e. the transpose of
            /// the cofactor matrix.
            ///
            /// This is equivalent to the inverse
            /// but without dividing by the determinant of the matrix,
            /// which can be useful in some contexts for better performance.
            ///
            /// One such case is when this matrix is interpreted as a
            /// a homogeneous transformation matrix, in which case uniform scaling will
            /// not affect the resulting projected 3d version of transformed points or
            /// vectors.
            #[inline]
            pub fn adjugate(&self) -> Self {
                let x = self.cols[1].cross(self.cols[2]);
                let y = self.cols[2].cross(self.cols[0]);
                let z = self.cols[0].cross(self.cols[1]);

                Self::new(x, y, z).transposed()
            }

            /// If this matrix is not currently invertable, this function will return
            /// an invalid inverse. This status is not checked by the library.
            #[inline]
            pub fn inverse(&mut self) {
                *self = self.inversed();
            }

            /// If this matrix is not currently invertable, this function will return
            /// an invalid inverse. This status is not checked by the library.
            #[inline]
            pub fn inversed(&self) -> Self {
                let adjugate = self.adjugate();
                let det = self.determinant();
                let inv_det = $t::splat(1.0) / det;

                inv_det * adjugate
            }

            #[inline]
            pub fn transpose(&mut self) {
                *self = self.transposed();
            }

            #[inline]
            pub fn transposed(&self) -> Self {
                let (x0, y0, z0) = self.cols[0].into();
                let (x1, y1, z1) = self.cols[1].into();
                let (x2, y2, z2) = self.cols[2].into();
                Self::new(
                    $vt::new(x0, x1, x2),
                    $vt::new(y0, y1, y2),
                    $vt::new(z0, z1, z2),
                )
            }

            /// Transform a Vec2 by self, interpreting it as a vector.
            #[inline]
            pub fn transform_vec2(&self, vec: $v2t) -> $v2t {
                (*self * vec.into_homogeneous_vector()).truncated()
            }

            /// Transform a Vec2 by self, interpreting it as a point.
            #[inline]
            pub fn transform_point2(&self, point: $v2t) -> $v2t {
                (*self * point.into_homogeneous_point()).normalized_homogeneous_point().truncated()
            }

            /// Get the [`core::alloc::Layout`] of `Self`
            #[inline]
            pub fn layout() -> alloc::alloc::Layout {
                alloc::alloc::Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<$t>()).unwrap()
            }

            /// Interpret `self` as a statically sized array of the base numeric type.
            #[inline]
            pub fn as_array(&self) -> &[$t; 9] {
                let ptr = self as *const $n as *const [$t; 9];
                unsafe { &*ptr }
            }

            /// Interpret `self` as a statically sized array of the base numeric type.
            #[inline]
            pub fn as_mut_array(&mut self) -> &mut [$t; 9] {
                let ptr = self as *mut $n as *mut [$t; 9];
                unsafe { &mut *ptr }
            }

            /// Interpret `self` as a statically sized array of the component (column) vectors.
            #[inline]
            pub fn as_component_array(&self) -> &[$vt; 3] {
                let ptr = self as *const $n as *const [$vt; 3];
                unsafe { &*ptr }
            }

            /// Interpret `self` as a statically sized array of the component (column) vectors.
            #[inline]
            pub fn as_mut_component_array(&mut self) -> &mut [$vt; 3] {
                let ptr = self as *mut $n as *mut [$vt; 3];
                unsafe { &mut *ptr }
            }

            /// Interpret `self` as a slice of the base numeric type.
            #[inline]
            pub fn as_slice(&self) -> &[$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const $t, 9)
                }
            }

            /// Interpret `self` as a slice of the component (column) vectors.
            #[inline]
            pub fn as_component_slice(&self) -> &[$vt] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const $vt, 3)
                }
            }

            /// Interpret `self` as a slice of bytes.
            #[inline]
            pub fn as_byte_slice(&self) -> &[u8] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const u8, 9 * std::mem::size_of::<$t>())
                }
            }

            /// Interpret `self` as a slice of the base numeric type.
            #[inline]
            pub fn as_mut_slice(&mut self) -> &mut [$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(self as *mut $n as *mut $t, 9)
                }
            }

            /// Interpret `self` as a slice of the component (column) vectors.
            #[inline]
            pub fn as_mut_component_slice(&mut self) -> &mut [$vt] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(self as *mut $n as *mut $vt, 3)
                }
            }

            /// Interpret `self` as a slice of bytes.
            #[inline]
            pub fn as_mut_byte_slice(&mut self) -> &mut [u8] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(self as *mut $n as *mut u8, 9 * std::mem::size_of::<$t>())
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
            pub const fn as_ptr(&self) -> *const $t {
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
                        (sa.x * oa.x) + (sb.x * oa.y) + (sc.x * oa.z),
                        (sa.y * oa.x) + (sb.y * oa.y) + (sc.y * oa.z),
                        (sa.z * oa.x) + (sb.z * oa.y) + (sc.z * oa.z),
                    ),
                    $vt::new(
                        (sa.x * ob.x) + (sb.x * ob.y) + (sc.x * ob.z),
                        (sa.y * ob.x) + (sb.y * ob.y) + (sc.y * ob.z),
                        (sa.z * ob.x) + (sb.z * ob.y) + (sc.z * ob.z),
                    ),
                    $vt::new(
                        (sa.x * oc.x) + (sb.x * oc.y) + (sc.x * oc.z),
                        (sa.y * oc.x) + (sb.y * oc.y) + (sc.y * oc.z),
                        (sa.z * oc.x) + (sb.z * oc.y) + (sc.z * oc.z),
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
                    (a.x * rhs.x) + (b.x * rhs.y) + (c.x * rhs.z),
                    (a.y * rhs.x) + (b.y * rhs.y) + (c.y * rhs.z),
                    (a.z * rhs.x) + (b.z * rhs.y) + (c.z * rhs.z),
                )
            }
        }

        impl Mul<$t> for $n {
            type Output = $n;
            #[inline]
            fn mul(self, rhs: $t) -> $n {
                $n::new(
                    self.cols[0] * rhs,
                    self.cols[1] * rhs,
                    self.cols[2] * rhs,
                )
            }
        }

        impl Mul<$n> for $t {
            type Output = $n;
            #[inline]
            fn mul(self, rhs: $n) -> $n {
                $n::new(
                    rhs.cols[0] * self,
                    rhs.cols[1] * self,
                    rhs.cols[2] * self,
                )
            }
        }

        impl Add for $n {
            type Output = Self;
            #[inline]
            fn add(self, rhs: $n) -> Self {
                $n::new(
                    self.cols[0] + rhs.cols[0],
                    self.cols[1] + rhs.cols[1],
                    self.cols[2] + rhs.cols[2],
                )
            }
        }

        impl AddAssign for $n {
            #[inline]
            fn add_assign(&mut self, rhs: $n) {
                *self = *self + rhs;
            }
        }

        impl From<[$t; 9]> for $n {
            #[inline]
            fn from(comps: [$t; 9]) -> Self {
                Self::new(
                    $vt::new(comps[0], comps[1], comps[2]),
                    $vt::new(comps[3], comps[4], comps[5]),
                    $vt::new(comps[6], comps[7], comps[8])
                )
            }
        }

        impl From<[[$t; 3]; 3]> for $n {
            #[inline]
            fn from(comps: [[$t; 3]; 3]) -> Self {
                Self::new(
                    $vt::new(comps[0][0], comps[0][1], comps[0][2]),
                    $vt::new(comps[1][0], comps[1][1], comps[1][2]),
                    $vt::new(comps[2][0], comps[2][1], comps[2][2])
                )
            }
        }

        impl From<$n> for [[$t; 3]; 3] {
            #[inline]
            fn from(mat3: $n) -> Self {
                [
                    [mat3.cols[0].x, mat3.cols[0].y, mat3.cols[0].z],
                    [mat3.cols[1].x, mat3.cols[1].y, mat3.cols[1].z],
                    [mat3.cols[2].x, mat3.cols[2].y, mat3.cols[2].z]
                ]
            }
        }

        impl From<&[$t; 9]> for $n {
            #[inline]
            fn from(comps: &[$t; 9]) -> Self {
                Self::from(*comps)
            }
        }

        impl Index<usize> for $n {
            type Output = $vt;

            fn index(&self, index: usize) -> &Self::Output {
                &self.cols[index]
            }
        }

        impl IndexMut<usize> for $n {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.cols[index]
            }
        }
        )+
    }
}

mat3s!(
    Mat3 => Rotor3, Bivec3, Mat4, Vec4, Vec2, Vec3, f32,
    Mat3x4 => Rotor3x4, Bivec3x4, Mat4x4, Vec4x4, Vec2x4, Vec3x4, f32x4,
    Mat3x8 => Rotor3x8, Bivec3x8, Mat4x8, Vec4x8, Vec2x8, Vec3x8, f32x8
);

#[cfg(feature = "f64")]
mat3s!(
    DMat3 => DRotor3, DBivec3, DMat4, DVec4, DVec2, DVec3, f64,
    DMat3x2 => DRotor3x2, DBivec3x2, DMat4x2, DVec4x2, DVec2x2, DVec3x2, f64x2,
    DMat3x4 => DRotor3x4, DBivec3x4, DMat4x4, DVec4x4, DVec2x4, DVec3x4, f64x4
);

macro_rules! impl_mat3 {
    ($($mt:ident, $t:ident, $rt:ident, $bt:ident),+) => {
        $(impl $mt {
            /// If `self` is a rotation matrix, return a `Rotor3` representing the same rotation.
            ///
            /// If `self` is not a rotation matrix, the returned value is a `Rotor3` with undefied
            /// properties. The fact that `self` is a rotation matrix is not checked by the
            /// library.
            pub fn into_rotor3(self) -> $rt {
                // Adapted from http://www.euclideanspace.com/maths/geometry/rotations/conversions/matrixToQuaternion/
                let w = ($t::splat(1.0) + self[0][0] + self[1][1] + self[2][2]).max($t::splat(0.0)).sqrt() * $t::splat(0.5);

                let yz = {
                    let s = ($t::splat(1.0) + self[0][0] - self[1][1] - self[2][2]).max($t::splat(0.0)).sqrt() * $t::splat(0.5);
                    s.copysign(self[2][1] - self[1][2])
                };

                let xz = {
                    let s = ($t::splat(1.0) - self[0][0] + self[1][1] - self[2][2]).max($t::splat(0.0)).sqrt() * $t::splat(0.5);
                    s.copysign(self[2][0] - self[0][2])
                };

                let xy = {
                    let s = ($t::splat(1.0) - self[0][0] - self[1][1] + self[2][2]).max($t::splat(0.0)).sqrt() * $t::splat(0.5);
                    s.copysign(self[1][0] - self[0][1])
                };

                $rt::new(w, $bt::new(xy, xz, yz))
            }
        })+
    }
}

impl_mat3!(Mat3, f32, Rotor3, Bivec3);

#[cfg(feature = "f64")]
impl_mat3!(DMat3, f64, DRotor3, DBivec3);

macro_rules! impl_mat3_wide {
    ($($mt:ident => $t:ident, $rt:ident, $bt:ident),+) => {
        $(impl $mt {
            /// If `self` is a rotation matrix, return a `Rotor3` representing the same rotation.
            ///
            /// If `self` is not a rotation matrix, the returned value is a `Rotor3` with undefied
            /// properties. The fact that `self` is a rotation matrix is not checked by the
            /// library.
            pub fn into_rotor3(self) -> $rt {
                // Adapted from http://www.euclideanspace.com/maths/geometry/rotations/conversions/matrixToQuaternion/
                let w = ($t::splat(1.0) + self[0][0] + self[1][1] + self[2][2]).max($t::splat(0.0)).sqrt() * $t::splat(0.5);

                let yz = {
                    let s = ($t::splat(1.0) + self[0][0] - self[1][1] - self[2][2]).max($t::splat(0.0)).sqrt() * $t::splat(0.5);
                    s.flip_signs(self[2][1] - self[1][2])
                };

                let xz = {
                    let s = ($t::splat(1.0) - self[0][0] + self[1][1] - self[2][2]).max($t::splat(0.0)).sqrt() * $t::splat(0.5);
                    s.flip_signs(self[2][0] - self[0][2])
                };

                let xy = {
                    let s = ($t::splat(1.0) - self[0][0] - self[1][1] + self[2][2]).max($t::splat(0.0)).sqrt() * $t::splat(0.5);
                    s.flip_signs(self[1][0] - self[0][1])
                };

                $rt::new(w, $bt::new(xy, xz, yz))
            }
        })+
    }
}

impl_mat3_wide!(Mat3x4 => f32x4, Rotor3x4, Bivec3x4,
                Mat3x8 => f32x8, Rotor3x8, Bivec3x8);

#[cfg(feature = "f64")]
impl_mat3_wide!(DMat3x2 => f64x2, DRotor3x2, DBivec3x2,
                DMat3x4 => f64x4, DRotor3x4, DBivec3x4);

macro_rules! mat4s {
    ($($n:ident => $rt:ident, $bt:ident, $vt:ident, $v3t:ident, $m3t:ident, $i3t:ident, $t:ident),+) => {
        $(/// A 4x4 square matrix.
        ///
        /// Useful for performing linear transformations (rotation, scaling) on 4d vectors,
        /// or for performing arbitrary transformations (linear + translation, projection, etc)
        /// on homogeneous 3d vectors.
        ///
        /// Note that most constructors assume that the matrix will be used as a homogeneous 3d
        /// transformation matrix.
        #[derive(Clone, Copy, Debug, PartialEq)]
        #[repr(C)]
        pub struct $n {
            pub cols: [$vt; 4],
        }

        derive_default_identity!($n);

        impl $n {
            #[inline]
            pub const fn new(col1: $vt, col2: $vt, col3: $vt, col4: $vt) -> Self {
                $n {
                    cols: [col1, col2, col3, col4],
                }
            }

            #[inline]
            pub fn identity() -> Self {
                Self::new(
                    $vt::new($t::splat(1.0), $t::splat(0.0), $t::splat(0.0), $t::splat(0.0)),
                    $vt::new($t::splat(0.0), $t::splat(1.0), $t::splat(0.0), $t::splat(0.0)),
                    $vt::new($t::splat(0.0), $t::splat(0.0), $t::splat(1.0), $t::splat(0.0)),
                    $vt::new($t::splat(0.0), $t::splat(0.0), $t::splat(0.0), $t::splat(1.0)))
            }

            /// Assumes homogeneous 3d coordinates.
            #[inline]
            pub fn from_translation(trans: $v3t) -> Self {
                Self::new(
                    $vt::new($t::splat(1.0), $t::splat(0.0), $t::splat(0.0), $t::splat(0.0)),
                    $vt::new($t::splat(0.0), $t::splat(1.0), $t::splat(0.0), $t::splat(0.0)),
                    $vt::new($t::splat(0.0), $t::splat(0.0), $t::splat(1.0), $t::splat(0.0)),
                    $vt::new(trans.x, trans.y, trans.z, $t::splat(1.0)))
            }

            /// Assumes homogeneous 3d coordinates.
            #[inline]
            pub fn from_scale(scale: $t) -> Self {
                let zero = $t::splat(0.0);
                Self::new(
                    $vt::new(scale, zero, zero, zero),
                    $vt::new(zero, scale, zero, zero),
                    $vt::new(zero, zero, scale, zero),
                    $vt::new(zero, zero, zero, $t::splat(1.0)),
                )
            }

            /// Assumes homogeneous 3d coordinates.
            #[inline]
            pub fn from_nonuniform_scale(scale: $v3t) -> Self {
                let zero = $t::splat(0.0);
                Self::new(
                    $vt::new(scale.x, zero, zero, zero),
                    $vt::new(zero, scale.y, zero, zero),
                    $vt::new(zero, zero, scale.z, zero),
                    $vt::new(zero, zero, zero, $t::splat(1.0)),
                )
            }

            /// Full 4d diagonal matrix.
            #[inline]
            pub fn from_scale_4d(scale: $t) -> Self {
                let zero = $t::splat(0.0);
                Self::new(
                    $vt::new(scale, zero, zero, zero),
                    $vt::new(zero, scale, zero, zero),
                    $vt::new(zero, zero, scale, zero),
                    $vt::new(zero, zero, zero, scale),
                )
            }

            /// Full 4d nonuniform scaling matrix.
            #[inline]
            pub fn from_nonuniform_scale_4d(scale: $vt) -> Self {
                let zero = $t::splat(0.0);
                Self::new(
                    $vt::new(scale.x, zero, zero, zero),
                    $vt::new(zero, scale.y, zero, zero),
                    $vt::new(zero, zero, scale.z, zero),
                    $vt::new(zero, zero, zero, scale.w),
                )
            }

            /// Angles are applied in the order roll -> pitch -> yaw
            ///
            /// - Roll is rotation inside the xy plane ("around the z axis")
            /// - Pitch is rotation inside the yz plane ("around the x axis")
            /// - Yaw is rotation inside the xz plane ("around the y axis")
            ///
            /// Assumes homogeneous 3d coordinates.
            ///
            /// **Important: This function assumes a right-handed, y-up coordinate space** where:
            /// * +X axis points *right*
            /// * +Y axis points *up*
            /// * +Z axis points *towards the viewer* (i.e. out of the screen)
            ///
            /// This means that you may see unexpected behavior when used with OpenGL or DirectX
            /// as they use a different coordinate system. You should use the appropriate
            /// projection matrix in ```projection``` module to fit your use case to remedy this.
            #[inline]
            pub fn from_euler_angles(roll: $t, pitch: $t, yaw: $t) -> Self {
                let (sin_yaw, cos_yaw) = yaw.sin_cos();
                let (sin_pitch, cos_pitch) = pitch.sin_cos();
                let (sin_roll, cos_roll) = roll.sin_cos();

                let zero = $t::splat(0.0);

                let m00 = cos_yaw * cos_roll + sin_pitch * sin_yaw * sin_roll;
                let m10 = cos_pitch * sin_roll;
                let m20 = -cos_roll * sin_yaw + cos_yaw * sin_pitch * sin_roll;
                let m01 = cos_roll * sin_pitch * sin_yaw - cos_yaw * sin_roll;
                let m11 = cos_pitch * cos_roll;
                let m21 = cos_yaw * cos_roll * sin_pitch + sin_yaw * sin_roll;
                let m02 = cos_pitch * sin_yaw;
                let m12 = -sin_pitch;
                let m22 = cos_pitch * cos_yaw;

                // think transposed as arguments are columns
                Self::new(
                    $vt::new(m00, m10, m20, zero),
                    $vt::new(m01, m11, m21, zero),
                    $vt::new(m02, m12, m22, zero),
                    $vt::new(zero, zero, zero, $t::splat(1.0))
                )
            }

            /// Create a new rotation matrix from a rotation "around the x axis". This is
            /// here as a convenience function for users coming from other libraries; it is
            /// more proper to think of this as a rotation *in the yz plane*.
            ///
            /// Assumes homogeneous 3d coordinates.
            #[inline]
            pub fn from_rotation_x(angle: $t) -> Self {
                let (sin, cos) = angle.sin_cos();
                let zero = $t::splat(0.0);
                let one = $t::splat(1.0);

                // think transposed as arguments are columns
                Self::new(
                    $vt::new(one, zero, zero, zero),
                    $vt::new(zero, cos, sin, zero),
                    $vt::new(zero, -sin, cos, zero),
                    $vt::new(zero, zero, zero, one),
                )
            }

            /// Create a new rotation matrix from a rotation "around the y axis". This is
            /// here as a convenience function for users coming from other libraries; it is
            /// more proper to think of this as a rotation *in the xz plane*.
            ///
            /// Assumes homogeneous 3d coordinates.
            #[inline]
            pub fn from_rotation_y(angle: $t) -> Self {
                let (sin, cos) = angle.sin_cos();
                let zero = $t::splat(0.0);
                let one = $t::splat(1.0);

                // think transposed as arguments are columns
                Self::new(
                    $vt::new(cos, zero, -sin, zero),
                    $vt::new(zero, one, zero, zero),
                    $vt::new(sin, zero, cos, zero),
                    $vt::new(zero, zero, zero, one),
                )
            }

            /// Create a new rotation matrix from a rotation "around the z axis". This is
            /// here as a convenience function for users coming from other libraries; it is
            /// more proper to think of this as a rotation *in the xy plane*.
            ///
            /// Assumes homogeneous 3d coordinates.
            #[inline]
            pub fn from_rotation_z(angle: $t) -> Self {
                let (sin, cos) = angle.sin_cos();
                let zero = $t::splat(0.0);
                let one = $t::splat(1.0);

                // think transposed as arguments are columns
                Self::new(
                    $vt::new(cos, sin, zero, zero),
                    $vt::new(-sin, cos, zero, zero),
                    $vt::new(zero, zero, one, zero),
                    $vt::new(zero, zero, zero, one),
                )
            }

            /// Create a new rotation matrix from a rotation around the given axis.
            /// The axis will be interpreted as a 3d vector.
            /// This is here as a convenience function for users coming from other libraries.
            ///
            /// Assumes homogeneous 3d coordinates.
            #[inline]
            pub fn from_rotation_around(axis: $vt, angle: $t) -> Self {
                let (sin, cos) = angle.sin_cos();
                let zero = $t::splat(0.0);
                let one = $t::splat(1.0);
                let mul = one - cos;


                let x_sin = axis.x * sin;
                let y_sin = axis.y * sin;
                let z_sin = axis.z * sin;

                let xy_mul = axis.x * axis.y * mul;
                let xz_mul = axis.x * axis.z * mul;
                let yz_mul = axis.y * axis.z * mul;

                let m00 = (axis.x * axis.x).mul_add(mul, cos);
                let m10 = xy_mul + z_sin;
                let m20 = xz_mul - y_sin;
                let m01 = xy_mul - z_sin;
                let m11 = (axis.y * axis.y).mul_add(mul, cos);
                let m21 = yz_mul + x_sin;
                let m02 = xz_mul + y_sin;
                let m12 = yz_mul - x_sin;
                let m22 = (axis.z * axis.z).mul_add(mul, cos);

                // think transposed as arguments are columns
                Self::new(
                    $vt::new(m00, m10, m20, zero),
                    $vt::new(m01, m11, m21, zero),
                    $vt::new(m02, m12, m22, zero),
                    $vt::new(zero, zero, zero, one),
                )
            }

            /// Construct a rotation matrix given a bivector which defines a plane and rotation orientation,
            /// and a rotation angle.
            ///
            /// `plane` must be normalized!
            ///
            /// This is the equivalent of an axis-angle rotation.
            ///
            /// Assumes homogeneous 3d coordinates.
            #[inline]
            pub fn from_angle_plane(angle: $t, plane: $bt) -> Self {
                $rt::from_angle_plane(angle, plane).into_matrix().into_homogeneous()
            }

            /// Assumes homogeneous 3d coordinates.
            pub fn translate(&mut self, translation: &$v3t) {
                self[3].x += translation.x;
                self[3].y += translation.y;
                self[3].z += translation.z;
            }

            /// Assumes homogeneous 3d coordinates.
            pub fn translated(&self, translation: &$v3t) -> Self {
                let mut res = *self;
                res.translate(translation);

                res
            }

            /// Constructs a 'look-at' matrix from an eye position, a focus position to look towards,
            /// and a vector that defines the 'up' direction.
            ///
            /// This function assumes a right-handed, y-up coordinate space.
            #[inline]
            pub fn look_at(eye: $v3t, at: $v3t, up: $v3t) -> Self {
                let f = (at - eye).normalized();
                let r = f.cross(up).normalized();
                let u = r.cross(f);
                Self::new(
                    $vt::new(r.x, u.x, -f.x, $t::splat(0.0)),
                    $vt::new(r.y, u.y, -f.y, $t::splat(0.0)),
                    $vt::new(r.z, u.z, -f.z, $t::splat(0.0)),
                    $vt::new(-r.dot(eye), -u.dot(eye), f.dot(eye), $t::splat(1.0))
                )
            }

            /// Constructs a 'look-at' matrix from an eye position, a focus position to look towards,
            /// and a vector that defines the 'up' direction.
            ///
            /// This function assumes a *left*-handed, y-up coordinate space.
            #[inline]
            pub fn look_at_lh(eye: $v3t, at: $v3t, up: $v3t) -> Self {
                let f = (at - eye).normalized();
                let r = f.cross(up).normalized();
                let u = r.cross(f);
                Self::new(
                    $vt::new(r.x, u.x, f.x, $t::splat(0.0)),
                    $vt::new(r.y, u.y, f.y, $t::splat(0.0)),
                    $vt::new(r.z, u.z, f.z, $t::splat(0.0)),
                    $vt::new(-r.dot(eye), -u.dot(eye), -f.dot(eye), $t::splat(1.0))
                )
            }

            #[inline]
            pub fn transpose(&mut self) {
                *self = self.transposed();
            }

            #[inline]
            pub fn transposed(&self) -> Self {
                let (x0, y0, z0, w0) = self.cols[0].into();
                let (x1, y1, z1, w1) = self.cols[1].into();
                let (x2, y2, z2, w2) = self.cols[2].into();
                let (x3, y3, z3, w3) = self.cols[3].into();
                Self::new(
                    $vt::new(x0, x1, x2, x3),
                    $vt::new(y0, y1, y2, y3),
                    $vt::new(z0, z1, z2, z3),
                    $vt::new(w0, w1, w2, w3),
                )
            }

            /// If this matrix is not currently invertable, this function will return
            /// an invalid inverse. This status is not checked by the library.
            #[inline]
            pub fn inverse(&mut self) {
                *self = self.inversed();
            }

            #[inline]
            pub fn determinant(&self) -> $t {
                let (m00, m01, m02, m03) = self.cols[0].into();
                let (m10, m11, m12, m13) = self.cols[1].into();
                let (m20, m21, m22, m23) = self.cols[2].into();
                let (m30, m31, m32, m33) = self.cols[3].into();

                let a2323 = (m22 * m33) - (m23 * m32);
                let a1323 = (m21 * m33) - (m23 * m31);
                let a1223 = (m21 * m32) - (m22 * m31);
                let a0323 = (m20 * m33) - (m23 * m30);
                let a0223 = (m20 * m32) - (m22 * m30);
                let a0123 = (m20 * m31) - (m21 * m30);

                m00 * (m11 * a2323 - m12 * a1323 + m13 * a1223)
                    - m01 * (m10 * a2323 - m12 * a0323 + m13 * a0223)
                    + m02 * (m10 * a1323 - m11 * a0323 + m13 * a0123)
                    - m03 * (m10 * a1223 - m11 * a0223 + m12 * a0123)
            }

            /// The adjugate of this matrix, i.e. the transpose of
            /// the cofactor matrix.
            ///
            /// This is equivalent to the inverse
            /// but without dividing by the determinant of the matrix,
            /// which can be useful in some contexts for better performance.
            ///
            /// One such case is when this matrix is interpreted as a
            /// a homogeneous transformation matrix, in which case uniform scaling will
            /// not affect the resulting projected 3d version of transformed points or
            /// vectors.
            #[inline]
            pub fn adjugate(&self) -> Self {
                let (m00, m01, m02, m03) = self.cols[0].into();
                let (m10, m11, m12, m13) = self.cols[1].into();
                let (m20, m21, m22, m23) = self.cols[2].into();
                let (m30, m31, m32, m33) = self.cols[3].into();

                let coef00 = (m22 * m33) - (m32 * m23);
                let coef02 = (m12 * m33) - (m32 * m13);
                let coef03 = (m12 * m23) - (m22 * m13);

                let coef04 = (m21 * m33) - (m31 * m23);
                let coef06 = (m11 * m33) - (m31 * m13);
                let coef07 = (m11 * m23) - (m21 * m13);

                let coef08 = (m21 * m32) - (m31 * m22);
                let coef10 = (m11 * m32) - (m31 * m12);
                let coef11 = (m11 * m22) - (m21 * m12);

                let coef12 = (m20 * m33) - (m30 * m23);
                let coef14 = (m10 * m33) - (m30 * m13);
                let coef15 = (m10 * m23) - (m20 * m13);

                let coef16 = (m20 * m32) - (m30 * m22);
                let coef18 = (m10 * m32) - (m30 * m12);
                let coef19 = (m10 * m22) - (m20 * m12);

                let coef20 = (m20 * m31) - (m30 * m21);
                let coef22 = (m10 * m31) - (m30 * m11);
                let coef23 = (m10 * m21) - (m20 * m11);

                let fac0 = $vt::new(coef00, coef00, coef02, coef03);
                let fac1 = $vt::new(coef04, coef04, coef06, coef07);
                let fac2 = $vt::new(coef08, coef08, coef10, coef11);
                let fac3 = $vt::new(coef12, coef12, coef14, coef15);
                let fac4 = $vt::new(coef16, coef16, coef18, coef19);
                let fac5 = $vt::new(coef20, coef20, coef22, coef23);

                let vec0 = $vt::new(m10, m00, m00, m00);
                let vec1 = $vt::new(m11, m01, m01, m01);
                let vec2 = $vt::new(m12, m02, m02, m02);
                let vec3 = $vt::new(m13, m03, m03, m03);

                let inv0 = (vec1 * fac0) - (vec2 * fac1) + (vec3 * fac2);
                let inv1 = (vec0 * fac0) - (vec2 * fac3) + (vec3 * fac4);
                let inv2 = (vec0 * fac1) - (vec1 * fac3) + (vec3 * fac5);
                let inv3 = (vec0 * fac2) - (vec1 * fac4) + (vec2 * fac5);

                let sign_a = $vt::new($t::splat(1.0), $t::splat(-1.0), $t::splat(1.0), $t::splat(-1.0));
                let sign_b = $vt::new($t::splat(-1.0), $t::splat(1.0), $t::splat(-1.0), $t::splat(1.0));

                Self {
                    cols: [
                        inv0 * sign_a,
                        inv1 * sign_b,
                        inv2 * sign_a,
                        inv3 * sign_b,
                    ]
                }
            }


            /// If this matrix is not currently invertable, this function will return
            /// an invalid inverse. This status is not checked by the library.
            #[inline]
            pub fn inversed(&self) -> Self {
                let adjugate = self.adjugate();

                let row0 = $vt::new(
                    adjugate.cols[0].x,
                    adjugate.cols[1].x,
                    adjugate.cols[2].x,
                    adjugate.cols[3].x,
                );

                let dot0 = self.cols[0] * row0;
                let dot1 = dot0.x + dot0.y + dot0.z + dot0.w;

                let rcp_det = $t::splat(1.0) / dot1;
                adjugate * rcp_det
            }

            /// Transform a Vec3 by self, interpreting it as a vector.
            #[inline]
            pub fn transform_vec3(&self, vec: $v3t) -> $v3t {
                (*self * vec.into_homogeneous_vector()).truncated()
            }

            /// Transform a Vec3 by self, interpreting it as a point.
            #[inline]
            pub fn transform_point3(&self, point: $v3t) -> $v3t {
                (*self * point.into_homogeneous_point()).normalized_homogeneous_point().truncated()
            }

            /// If `self` represents an affine transformation, return its translation components.
            /// Otherwise, the returned value has undefined properties.
            #[inline]
            pub fn extract_translation(&self) -> $v3t {
                self.cols[3].truncated()
            }

            /// If the 3x3 left upper block of `self` is a rotation, return the corresponding
            /// rotor. Otherwise, the returned value is a `Rotor3` with undefined properties.
            pub fn extract_rotation(&self) -> $rt {
                self.truncate().into_rotor3()
            }

            /// If self represents an `Isometry3` (i.e. self is a product of the from `T * R` where
            /// `T` is a translation and `R` a rotation), return the isometry
            ///
            /// If `self` does not represent an isometry, the returned value has undefined
            /// properties.
            #[allow(clippy::wrong_self_convention)]
            pub fn into_isometry(&self) -> $i3t {
                $i3t::new(self.extract_translation(), self.extract_rotation())
            }

            /// Truncate `self` to a matrix consisting of the 3x3 left upper block.
            /// If you need a rotation, consider using [`Self::extract_rotation()`] instead.
            pub fn truncate(&self) -> $m3t {
                $m3t::new(
                    self.cols[0].truncated(),
                    self.cols[1].truncated(),
                    self.cols[2].truncated(),
                )
            }

            /// Get the [`core::alloc::Layout`] of `Self`
            #[inline]
            pub fn layout() -> alloc::alloc::Layout {
                alloc::alloc::Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<$t>()).unwrap()
            }

            /// Interpret `self` as a statically sized array of the base numeric type.
            #[inline]
            pub fn as_array(&self) -> &[$t; 16] {
                let ptr = self as *const $n as *const [$t; 16];
                unsafe { &*ptr }
            }

            /// Interpret `self` as a statically sized array of the base numeric type.
            #[inline]
            pub fn as_mut_array(&mut self) -> &mut [$t; 16] {
                let ptr = self as *mut $n as *mut [$t; 16];
                unsafe { &mut *ptr }
            }

            /// Interpret `self` as a statically sized array of its component (column) vectors.
            #[inline]
            pub fn as_component_array(&self) -> &[$vt; 4] {
                let ptr = self as *const $n as *const [$vt; 4];
                unsafe { &*ptr }
            }

            /// Interpret `self` as a statically sized array of its component (column) vectors.
            #[inline]
            pub fn as_mut_component_array(&mut self) -> &mut [$vt; 4] {
                let ptr = self as *mut $n as *mut [$vt; 4];
                unsafe { &mut *ptr }
            }

            /// Interpret `self` as a slice of the base numeric type.
            #[inline]
            pub fn as_slice(&self) -> &[$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const $t, 16)
                }
            }

            /// Interpret `self` as a slice of the base numeric type.
            #[inline]
            pub fn as_mut_slice(&mut self) -> &mut [$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(self as *mut $n as *mut $t, 16)
                }
            }

            /// Interpret `self` as a slice of the component (column) vectors
            #[inline]
            pub fn as_component_slice(&self) -> &[$vt] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const $vt, 4)
                }
            }

            /// Interpret `self` as a slice of the component (column) vectors
            #[inline]
            pub fn as_mut_component_slice(&mut self) -> &mut [$vt] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(self as *mut $n as *mut $vt, 4)
                }
            }

            /// Interpret `self` as a slice of bytes
            #[inline]
            pub fn as_byte_slice(&self) -> &[u8] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const u8, 16 * std::mem::size_of::<$t>())
                }
            }

            /// Interpret `self` as a slice of bytes
            #[inline]
            pub fn as_mut_byte_slice(&mut self) -> &mut [u8] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(self as *mut $n as *mut u8, 16 * std::mem::size_of::<$t>())
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
            pub const fn as_ptr(&self) -> *const $t {
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
                        (sa.x * oa.x) + (sb.x * oa.y) + (sc.x * oa.z) + (sd.x * oa.w),
                        (sa.y * oa.x) + (sb.y * oa.y) + (sc.y * oa.z) + (sd.y * oa.w),
                        (sa.z * oa.x) + (sb.z * oa.y) + (sc.z * oa.z) + (sd.z * oa.w),
                        (sa.w * oa.x) + (sb.w * oa.y) + (sc.w * oa.z) + (sd.w * oa.w),
                    ),
                    $vt::new(
                        (sa.x * ob.x) + (sb.x * ob.y) + (sc.x * ob.z) + (sd.x * ob.w),
                        (sa.y * ob.x) + (sb.y * ob.y) + (sc.y * ob.z) + (sd.y * ob.w),
                        (sa.z * ob.x) + (sb.z * ob.y) + (sc.z * ob.z) + (sd.z * ob.w),
                        (sa.w * ob.x) + (sb.w * ob.y) + (sc.w * ob.z) + (sd.w * ob.w),
                    ),
                    $vt::new(
                        (sa.x * oc.x) + (sb.x * oc.y) + (sc.x * oc.z) + (sd.x * oc.w),
                        (sa.y * oc.x) + (sb.y * oc.y) + (sc.y * oc.z) + (sd.y * oc.w),
                        (sa.z * oc.x) + (sb.z * oc.y) + (sc.z * oc.z) + (sd.z * oc.w),
                        (sa.w * oc.x) + (sb.w * oc.y) + (sc.w * oc.z) + (sd.w * oc.w),
                    ),
                    $vt::new(
                        (sa.x * od.x) + (sb.x * od.y) + (sc.x * od.z) + (sd.x * od.w),
                        (sa.y * od.x) + (sb.y * od.y) + (sc.y * od.z) + (sd.y * od.w),
                        (sa.z * od.x) + (sb.z * od.y) + (sc.z * od.z) + (sd.z * od.w),
                        (sa.w * od.x) + (sb.w * od.y) + (sc.w * od.z) + (sd.w * od.w),
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
                    a.w * rhs.x + b.w * rhs.y + c.w * rhs.z + d.w * rhs.w,
                )
            }
        }

        impl Mul<$t> for $n {
            type Output = $n;
            #[inline]
            fn mul(self, rhs: $t) -> $n {
                $n::new(
                    self.cols[0] * rhs,
                    self.cols[1] * rhs,
                    self.cols[2] * rhs,
                    self.cols[3] * rhs,
                )
            }
        }

        impl Mul<$n> for $t {
            type Output = $n;
            #[inline]
            fn mul(self, rhs: $n) -> $n {
                $n::new(
                    rhs.cols[0] * self,
                    rhs.cols[1] * self,
                    rhs.cols[2] * self,
                    rhs.cols[3] * self,
                )
            }
        }

        impl Add for $n {
            type Output = Self;
            #[inline]
            fn add(self, rhs: $n) -> Self {
                $n::new(
                    self.cols[0] + rhs.cols[0],
                    self.cols[1] + rhs.cols[1],
                    self.cols[2] + rhs.cols[2],
                    self.cols[3] + rhs.cols[3],
                )
            }
        }

        impl AddAssign for $n {
            #[inline]
            fn add_assign(&mut self, rhs: $n) {
                *self = *self + rhs;
            }
        }

        impl From<[$t; 16]> for $n {
            #[inline]
            fn from(comps: [$t; 16]) -> Self {
                Self::new(
                    $vt::new(comps[0], comps[1], comps[2], comps[3]),
                    $vt::new(comps[4], comps[5], comps[6], comps[7]),
                    $vt::new(comps[8], comps[9], comps[10], comps[11]),
                    $vt::new(comps[12], comps[13], comps[14], comps[15]),
                )
            }
        }

        impl From<[[$t; 4]; 4]> for $n {
            #[inline]
            fn from(comps: [[$t; 4]; 4]) -> Self {
                Self::new(
                    $vt::new(comps[0][0], comps[0][1], comps[0][2], comps[0][3]),
                    $vt::new(comps[1][0], comps[1][1], comps[1][2], comps[1][3]),
                    $vt::new(comps[2][0], comps[2][1], comps[2][2], comps[2][3]),
                    $vt::new(comps[3][0], comps[3][1], comps[3][2], comps[3][3])
                )
            }
        }

        impl From<$n> for [[$t; 4]; 4] {
            #[inline]
            fn from(mat4: $n) -> Self {
                [
                    [mat4.cols[0].x, mat4.cols[0].y, mat4.cols[0].z, mat4.cols[0].w],
                    [mat4.cols[1].x, mat4.cols[1].y, mat4.cols[1].z, mat4.cols[1].w],
                    [mat4.cols[2].x, mat4.cols[2].y, mat4.cols[2].z, mat4.cols[2].w],
                    [mat4.cols[3].x, mat4.cols[3].y, mat4.cols[3].z, mat4.cols[3].w]
                ]
            }
        }

        impl From<&[$t; 16]> for $n {
            #[inline]
            fn from(comps: &[$t; 16]) -> Self {
                Self::from(*comps)
            }
        }

        impl Index<usize> for $n {
            type Output = $vt;

            fn index(&self, index: usize) -> &Self::Output {
                &self.cols[index]
            }
        }

        impl IndexMut<usize> for $n {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.cols[index]
            }
        }

        )+
    }
}

mat4s!(
    Mat4 => Rotor3, Bivec3, Vec4, Vec3, Mat3, Isometry3, f32,
    Mat4x4 => Rotor3x4, Bivec3x4, Vec4x4, Vec3x4, Mat3x4, Isometry3x4, f32x4,
    Mat4x8 => Rotor3x8, Bivec3x8, Vec4x8, Vec3x8, Mat3x8, Isometry3x8, f32x8
);

#[cfg(feature = "f64")]
mat4s!(
    DMat4 => DRotor3, DBivec3, DVec4, DVec3, DMat3, DIsometry3, f64,
    DMat4x2 => DRotor3x2, DBivec3x2, DVec4x2, DVec3x2, DMat3x2, DIsometry3x2, f64x2,
    DMat4x4 => DRotor3x4, DBivec3x4, DVec4x4, DVec3x4, DMat3x4, DIsometry3x4, f64x4
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::*;

    /* TODO:
    Re-enable these. The current way that Matrix3::into_rotor() works sometimes fails these
    edge cases based on rounding error accumulated from the round trip due to the way it uses

    use std::f32::consts::FRAC_PI_2;
    use std::f32::consts::PI;

    copysign()
        #[test]
        pub fn mat3_to_rotor_corner_cases(){
            for i in 0..64 {
                let alpha = {
                    match i % 4 {
                        0 => -FRAC_PI_2,
                        1 => 0.,
                        2 => FRAC_PI_2,
                        3 => PI,
                        _ => unreachable!()
                    }
                };
                let beta = {
                    match (i / 4) % 4 {
                        0 => -FRAC_PI_2,
                        1 => 0.,
                        2 => FRAC_PI_2,
                        3 => PI,
                        _ => unreachable!()
                    }
                };
                let gamma = {
                    match (i / 16) % 4 {
                        0 => -FRAC_PI_2,
                        1 => 0.,
                        2 => FRAC_PI_2,
                        3 => PI,
                        _ => unreachable!()
                    }
                };
                println!("roll {}, pitch {}, yaw {}", alpha, beta, gamma);
                let rotor = Rotor3::from_euler_angles(alpha, beta, gamma);
                let mat = rotor.into_matrix();
                let rotor2 = mat.into_rotor3();
                assert!(rotor.eq_eps(rotor2));
                let xr = Vec3::unit_x().rotated_by(rotor);
                let xr2 = Vec3::unit_x().rotated_by(rotor2);
                assert!(xr.eq_eps(xr2));

                let yr = Vec3::unit_y().rotated_by(rotor);
                let yr2 = Vec3::unit_y().rotated_by(rotor2);
                assert!(yr.eq_eps(yr2));

                let zr = Vec3::unit_z().rotated_by(rotor);
                let zr2 = Vec3::unit_z().rotated_by(rotor2);
                assert!(zr.eq_eps(zr2));
            }


        }*/

    #[test]
    pub fn isometry_roundtrip() {
        let a = Vec3::new(1.0, 2.0, -5.0).normalized();
        let b = Vec3::new(1.0, 1.0, 1.0).normalized();
        let c = Vec3::new(2.0, 3.0, -3.0).normalized();
        let r_ab = Rotor3::from_rotation_between(a, b);
        let iso = Isometry3::new(c, r_ab);
        let iso_mat4 = iso.into_homogeneous_matrix();
        let iso_ = iso_mat4.into_isometry();
        assert!(iso_.translation.eq_eps(c));
        assert!(iso_.rotation.eq_eps(r_ab));
    }

    #[test]
    pub fn test_euler_angle_conversion() {
        let roll = 0.4;
        let yaw = 0.3;
        let pitch = 0.2;

        let mat1 = Mat3::from_euler_angles(roll, pitch, yaw);
        let mat2 =
            Mat3::from_rotation_y(yaw) * Mat3::from_rotation_x(pitch) * Mat3::from_rotation_z(roll);
        assert_eq!(mat1[0], mat2[0]);
        assert_eq!(mat1[1], mat2[1]);
        assert_eq!(mat1[2], mat2[2]);

        let mat3 = Mat4::from_euler_angles(roll, pitch, yaw);
        let mat4 =
            Mat4::from_rotation_y(yaw) * Mat4::from_rotation_x(pitch) * Mat4::from_rotation_z(roll);
        assert_eq!(mat3[0], mat4[0]);
        assert_eq!(mat3[1], mat4[1]);
        assert_eq!(mat3[2], mat4[2]);
        assert_eq!(mat3[3], mat4[3]);
    }
}
