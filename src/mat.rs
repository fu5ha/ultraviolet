//! Square matrices.
use std::ops::*;

use crate::*;

macro_rules! mat2s {
    ($($n:ident => $m3t:ident, $v3t:ident, $vt:ident, $t:ident),+) => {
        $(/// A 2x2 square matrix.
        ///
        /// Useful for performing linear transformations (rotation, scaling) on 2d vectors.
        #[derive(Clone, Copy, Debug)]
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
                self.cols[0].x.mul_add(self.cols[1].y, -(self.cols[1].x * self.cols[0].y))
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

                Self::new(
                    $vt::new(self.cols[1].y * inv_det, -self.cols[0].y * inv_det),
                    $vt::new(-self.cols[1].x * inv_det, self.cols[0].x * inv_det),
                )
            }

            #[inline]
            pub fn layout() -> alloc::alloc::Layout {
                alloc::alloc::Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<$vt>()).unwrap()
            }

            #[inline]
            pub fn as_array(&self) -> &[$t; 4] {
                use std::convert::TryInto;
                self.as_slice().try_into().unwrap()
            }

            #[inline]
            pub fn as_component_array(&self) -> &[$vt; 2] {
                use std::convert::TryInto;
                self.as_component_slice().try_into().unwrap()
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
            pub fn as_component_slice(&self) -> &[$vt] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const $vt, 2)
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
            pub fn as_mut_component_slice(&mut self) -> &mut [$vt] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(self as *mut $n as *mut $vt, 2)
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
                        sa.x.mul_add(oa.x, sb.x * oa.y),
                        sa.y.mul_add(oa.x, sb.y * oa.y),
                    ),
                    $vt::new(
                        sa.x.mul_add(ob.x, sb.x * ob.y),
                        sa.y.mul_add(ob.x, sb.y * ob.y),
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
                    a.x.mul_add(rhs.x, b.x * rhs.y),
                    a.y.mul_add(rhs.x, b.y * rhs.y),
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
                    $vt::new(comps[0][0], comps[0][0]),
                    $vt::new(comps[1][1], comps[1][1])
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

macro_rules! impl_partialeq_mat2 {
    ($($mt:ident),+) => {
        $(impl PartialEq for $mt {
            fn eq(&self, other: &Self) -> bool {
                self.cols[0] == other.cols[0] && self.cols[1] == other.cols[1]
            }
        })+
    }
}

impl_partialeq_mat2!(Mat2);

#[cfg(feature = "f64")]
impl_partialeq_mat2!(DMat2);

macro_rules! mat3s {
    ($($n:ident => $rt:ident, $bt:ident, $m4t:ident, $v4t:ident, $v2t:ident, $vt:ident, $t:ident),+) => {
        $(/// A 3x3 square matrix.
        ///
        /// Useful for performing linear transformations (rotation, scaling) on 3d vectors,
        /// or for performing arbitrary transformations (linear +   translation, projection, etc)
        /// on homogeneous 2d vectors
        #[derive(Clone, Copy, Debug)]
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
            pub fn from_nonuniform_scale_homogeneous(scale: $vt) -> Self {
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
                    $vt::new(s, -c, zero),
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
                let rotor = $rt::from_euler_angles(roll, pitch, yaw);
                rotor.into_matrix()
            }

            /// Create a new rotation matrix from a rotation "about the x axis". This is
            /// here as a convenience function for users coming from other libraries; it is
            /// more proper to think of this as a rotation *in the yz plane*.
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
            pub fn from_rotation_x(angle: $t) -> Self {
                // TODO: Easy optimization target.
                Self::from_euler_angles($t::splat(0.0), angle, $t::splat(0.0))
            }

            /// Create a new rotation matrix from a rotation "about the y axis". This is
            /// here as a convenience function for users coming from other libraries; it is
            /// more proper to think of this as a rotation *in the xz plane*.
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
            pub fn from_rotation_y(angle: $t) -> Self {
                // TODO: Easy optimization target.
                Self::from_euler_angles($t::splat(0.0), $t::splat(0.0), angle)
            }

            /// Create a new rotation matrix from a rotation "about the z axis". This is
            /// here as a convenience function for users coming from other libraries; it is
            /// more proper to think of this as a rotation *in the xy plane*.
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
            pub fn from_rotation_z(angle: $t) -> Self {
                // TODO: Easy optimization target.
                Self::from_euler_angles(angle, $t::splat(0.0), $t::splat(0.0))
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

            /// If this matrix is not currently invertable, this function will return
            /// an invalid inverse. This status is not checked by the library.
            #[inline]
            pub fn inverse(&mut self) {
                *self = self.transposed();
            }

            /// If this matrix is not currently invertable, this function will return
            /// an invalid inverse. This status is not checked by the library.
            #[inline]
            pub fn inversed(&self) -> Self {
                let x = self.cols[1].cross(self.cols[2]);
                let y = self.cols[2].cross(self.cols[0]);
                let z = self.cols[0].cross(self.cols[1]);
                let det = self.determinant();
                let inv_det = $t::splat(1.0) / det;

                Self::new(x * inv_det, y * inv_det, z * inv_det).transposed()
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

            #[inline]
            pub fn layout() -> alloc::alloc::Layout {
                alloc::alloc::Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<$t>()).unwrap()
            }

            #[inline]
            pub fn as_array(&self) -> &[$t; 9] {
                use std::convert::TryInto;
                self.as_slice().try_into().unwrap()
            }

            #[inline]
            pub fn as_component_array(&self) -> &[$vt; 3] {
                use std::convert::TryInto;
                self.as_component_slice().try_into().unwrap()
            }

            #[inline]
            pub fn as_slice(&self) -> &[$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const $t, 9)
                }
            }

            #[inline]
            pub fn as_component_slice(&self) -> &[$vt] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const $vt, 3)
                }
            }

            #[inline]
            pub fn as_byte_slice(&self) -> &[u8] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const u8, 9 * std::mem::size_of::<$t>())
                }
            }

            #[inline]
            pub fn as_mut_slice(&mut self) -> &mut [$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(self as *mut $n as *mut $t, 9)
                }
            }

            #[inline]
            pub fn as_mut_component_slice(&mut self) -> &mut [$vt] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(self as *mut $n as *mut $vt, 3)
                }
            }

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
                        sa.x.mul_add(oa.x, sb.x.mul_add(oa.y, sc.x * oa.z)),
                        sa.y.mul_add(oa.x, sb.y.mul_add(oa.y, sc.y * oa.z)),
                        sa.z.mul_add(oa.x, sb.z.mul_add(oa.y, sc.z * oa.z)),
                    ),
                    $vt::new(
                        sa.x.mul_add(ob.x, sb.x.mul_add(ob.y, sc.x * ob.z)),
                        sa.y.mul_add(ob.x, sb.y.mul_add(ob.y, sc.y * ob.z)),
                        sa.z.mul_add(ob.x, sb.z.mul_add(ob.y, sc.z * ob.z)),
                    ),
                    $vt::new(
                        sa.x.mul_add(oc.x, sb.x.mul_add(oc.y, sc.x * oc.z)),
                        sa.y.mul_add(oc.x, sb.y.mul_add(oc.y, sc.y * oc.z)),
                        sa.z.mul_add(oc.x, sb.z.mul_add(oc.y, sc.z * oc.z)),
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
                    a.x.mul_add(rhs.x, b.x.mul_add(rhs.y, c.x * rhs.z)),
                    a.y.mul_add(rhs.x, b.y.mul_add(rhs.y, c.y * rhs.z)),
                    a.z.mul_add(rhs.x, b.z.mul_add(rhs.y, c.z * rhs.z)),
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

macro_rules! impl_partialeq_mat3 {
    ($($mt:ident),+) => {
        $(impl PartialEq for $mt {
            fn eq(&self, other: &Self) -> bool {
                self.cols[0] == other.cols[0]
                    && self.cols[1] == other.cols[1]
                    && self.cols[2] == other.cols[2]
            }
        })+
    }
}

impl_partialeq_mat3!(Mat3);

#[cfg(feature = "f64")]
impl_partialeq_mat3!(DMat3);

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
                let w = ($t::splat(1.0) + self[0][0] + self[1][1] + self[2][2]).max($t::splat(0.0)).sqrt() / $t::splat(2.0);

                let yz = {
                    let s = ($t::splat(1.0) + self[0][0] - self[1][1] - self[2][2]).max($t::splat(0.0)).sqrt() / $t::splat(2.0);
                    s.copysign(self[2][1] - self[1][2])
                };

                let xz = {
                    let s = ($t::splat(1.0) - self[0][0] + self[1][1] - self[2][2]).max($t::splat(0.0)).sqrt() / $t::splat(2.0);
                    s.copysign(self[2][0] - self[0][2])
                };

                let xy = {
                    let s = ($t::splat(1.0) - self[0][0] - self[1][1] + self[2][2]).max($t::splat(0.0)).sqrt() / $t::splat(2.0);
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
                let w = ($t::splat(1.0) + self[0][0] + self[1][1] + self[2][2]).max($t::splat(0.0)).sqrt() / $t::splat(2.0);

                let yz = {
                    let s = ($t::splat(1.0) + self[0][0] - self[1][1] - self[2][2]).max($t::splat(0.0)).sqrt() / $t::splat(2.0);
                    s.flip_signs(self[2][1] - self[1][2])
                };

                let xz = {
                    let s = ($t::splat(1.0) - self[0][0] + self[1][1] - self[2][2]).max($t::splat(0.0)).sqrt() / $t::splat(2.0);
                    s.flip_signs(self[2][0] - self[0][2])
                };

                let xy = {
                    let s = ($t::splat(1.0) - self[0][0] - self[1][1] + self[2][2]).max($t::splat(0.0)).sqrt() / $t::splat(2.0);
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
impl_mat3_wide!(DMat3x4 => f64x4, DRotor3x4, DBivec3x4,
                DMat3x8 => f64x8, DRotor3x8, DBivec3x8);

macro_rules! mat4s {
    ($($n:ident => $rt:ident, $bt:ident, $vt:ident, $v3t:ident, $t:ident),+) => {
        $(/// A 4x4 square matrix.
        ///
        /// Useful for performing linear transformations (rotation, scaling) on 4d vectors,
        /// or for performing arbitrary transformations (linear + translation, projection, etc)
        /// on homogeneous 3d vectors.
        ///
        /// Note that most constructors assume that the matrix will be used as a homogeneous 3d
        /// transformation matrix.
        #[derive(Clone, Copy, Debug)]
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
                let rotor = $rt::from_euler_angles(roll, pitch, yaw);
                rotor.into_matrix().into_homogeneous()
            }

            /// Create a new rotation matrix from a rotation "about the x axis". This is
            /// here as a convenience function for users coming from other libraries; it is
            /// more proper to think of this as a rotation *in the yz plane*.
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
            pub fn from_rotation_x(angle: $t) -> Self {
                // TODO: Easy optimization target.
                Self::from_euler_angles($t::splat(0.0), angle, $t::splat(0.0))
            }

            /// Create a new rotation matrix from a rotation "about the y axis". This is
            /// here as a convenience function for users coming from other libraries; it is
            /// more proper to think of this as a rotation *in the xz plane*.
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
            pub fn from_rotation_y(angle: $t) -> Self {
                Self::from_euler_angles($t::splat(0.0), $t::splat(0.0), angle)
            }

            /// Create a new rotation matrix from a rotation "about the z axis". This is
            /// here as a convenience function for users coming from other libraries; it is
            /// more proper to think of this as a rotation *in the xy plane*.
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
            pub fn from_rotation_z(angle: $t) -> Self {
                // TODO: Easy optimization target.
                Self::from_euler_angles(angle, $t::splat(0.0), $t::splat(0.0))
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

                let a2323 = m22.mul_add(m33, -(m23 * m32));
                let a1323 = m21.mul_add(m33, -(m23 * m31));
                let a1223 = m21.mul_add(m32, -(m22 * m31));
                let a0323 = m20.mul_add(m33, -(m23 * m30));
                let a0223 = m20.mul_add(m32, -(m22 * m30));
                let a0123 = m20.mul_add(m31, -(m21 * m30));

                m00.mul_add(
                    m11.mul_add(a2323, -(m12.mul_add(a1323, -(m13 * a1223)))),
                    -(m01.mul_add(
                        m10.mul_add(a2323, -(m12.mul_add(a0323, -(m13 * a0223)))),
                        -(m02.mul_add(
                            m10.mul_add(a1323, -(m11.mul_add(a0323, -(m13 * a0123)))),
                            -(m03 * m10.mul_add(a1223, -(m11.mul_add(a0223, -(m12 * a0123)))))
                        ))
                    ))
                )
            }
            /// If this matrix is not currently invertable, this function will return
            /// an invalid inverse. This status is not checked by the library.
            #[inline]
            pub fn inversed(&self) -> Self {
                let (m00, m01, m02, m03) = self.cols[0].into();
                let (m10, m11, m12, m13) = self.cols[1].into();
                let (m20, m21, m22, m23) = self.cols[2].into();
                let (m30, m31, m32, m33) = self.cols[3].into();

                let coef00 = m22.mul_add(m33, -(m32 * m23));
                let coef02 = m12.mul_add(m33, -(m32 * m13));
                let coef03 = m12.mul_add(m23, -(m22 * m13));

                let coef04 = m21.mul_add(m33, -(m31 * m23));
                let coef06 = m11.mul_add(m33, -(m31 * m13));
                let coef07 = m11.mul_add(m23, -(m21 * m13));

                let coef08 = m21.mul_add(m32, -(m31 * m22));
                let coef10 = m11.mul_add(m32, -(m31 * m12));
                let coef11 = m11.mul_add(m22, -(m21 * m12));

                let coef12 = m20.mul_add(m33, -(m30 * m23));
                let coef14 = m10.mul_add(m33, -(m30 * m13));
                let coef15 = m10.mul_add(m23, -(m20 * m13));

                let coef16 = m20.mul_add(m32, -(m30 * m22));
                let coef18 = m10.mul_add(m32, -(m30 * m12));
                let coef19 = m10.mul_add(m22, -(m20 * m12));

                let coef20 = m20.mul_add(m31, -(m30 * m21));
                let coef22 = m10.mul_add(m31, -(m30 * m11));
                let coef23 = m10.mul_add(m21, -(m20 * m11));

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

                let inv0 = vec1.mul_add(fac0, -(vec2.mul_add(fac1, -(vec3 * fac2))));
                let inv1 = vec0.mul_add(fac0, -(vec2.mul_add(fac3, -(vec3 * fac4))));
                let inv2 = vec0.mul_add(fac1, -(vec1.mul_add(fac3, -(vec3 * fac5))));
                let inv3 = vec0.mul_add(fac2, -(vec1.mul_add(fac4, -(vec2 * fac5))));

                let sign_a = $vt::new($t::splat(1.0), $t::splat(-1.0), $t::splat(1.0), $t::splat(-1.0));
                let sign_b = $vt::new($t::splat(-1.0), $t::splat(1.0), $t::splat(-1.0), $t::splat(1.0));

                let inverse = Self {
                    cols: [
                        inv0 * sign_a,
                        inv1 * sign_b,
                        inv2 * sign_a,
                        inv3 * sign_b,
                    ]
                };

                let row0 = $vt::new(
                    inverse.cols[0].x,
                    inverse.cols[1].x,
                    inverse.cols[2].x,
                    inverse.cols[3].x,
                );

                let dot0 = self.cols[0] * row0;
                let dot1 = dot0.x + dot0.y + dot0.z + dot0.w;

                let rcp_det = $t::splat(1.0) / dot1;
                inverse * rcp_det
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

            #[inline]
            pub fn layout() -> alloc::alloc::Layout {
                alloc::alloc::Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<$t>()).unwrap()
            }

            #[inline]
            pub fn as_array(&self) -> &[$t; 16] {
                use std::convert::TryInto;
                self.as_slice().try_into().unwrap()
            }

            #[inline]
            pub fn as_component_array(&self) -> &[$vt; 4] {
                use std::convert::TryInto;
                self.as_component_slice().try_into().unwrap()
            }

            #[inline]
            pub fn as_slice(&self) -> &[$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const $t, 16)
                }
            }

            #[inline]
            pub fn as_component_slice(&self) -> &[$vt] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const $vt, 4)
                }
            }

            #[inline]
            pub fn as_byte_slice(&self) -> &[u8] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(self as *const $n as *const u8, 16 * std::mem::size_of::<$t>())
                }
            }

            #[inline]
            pub fn as_mut_slice(&mut self) -> &mut [$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(self as *mut $n as *mut $t, 16)
                }
            }

            #[inline]
            pub fn as_mut_component_slice(&mut self) -> &mut [$vt] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(self as *mut $n as *mut $vt, 4)
                }
            }

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
                        sa.x.mul_add(oa.x, sb.x.mul_add(oa.y, sc.x.mul_add(oa.z, sd.x * oa.w))),
                        sa.y.mul_add(oa.x, sb.y.mul_add(oa.y, sc.y.mul_add(oa.z, sd.y * oa.w))),
                        sa.z.mul_add(oa.x, sb.z.mul_add(oa.y, sc.z.mul_add(oa.z, sd.z * oa.w))),
                        sa.w.mul_add(oa.x, sb.w.mul_add(oa.y, sc.w.mul_add(oa.z, sd.w * oa.w))),
                    ),
                    $vt::new(
                        sa.x.mul_add(ob.x, sb.x.mul_add(ob.y, sc.x.mul_add(ob.z, sd.x * ob.w))),
                        sa.y.mul_add(ob.x, sb.y.mul_add(ob.y, sc.y.mul_add(ob.z, sd.y * ob.w))),
                        sa.z.mul_add(ob.x, sb.z.mul_add(ob.y, sc.z.mul_add(ob.z, sd.z * ob.w))),
                        sa.w.mul_add(ob.x, sb.w.mul_add(ob.y, sc.w.mul_add(ob.z, sd.w * ob.w))),
                    ),
                    $vt::new(
                        sa.x.mul_add(oc.x, sb.x.mul_add(oc.y, sc.x.mul_add(oc.z, sd.x * oc.w))),
                        sa.y.mul_add(oc.x, sb.y.mul_add(oc.y, sc.y.mul_add(oc.z, sd.y * oc.w))),
                        sa.z.mul_add(oc.x, sb.z.mul_add(oc.y, sc.z.mul_add(oc.z, sd.z * oc.w))),
                        sa.w.mul_add(oc.x, sb.w.mul_add(oc.y, sc.w.mul_add(oc.z, sd.w * oc.w))),
                    ),
                    $vt::new(
                        sa.x.mul_add(od.x, sb.x.mul_add(od.y, sc.x.mul_add(od.z, sd.x * od.w))),
                        sa.y.mul_add(od.x, sb.y.mul_add(od.y, sc.y.mul_add(od.z, sd.y * od.w))),
                        sa.z.mul_add(od.x, sb.z.mul_add(od.y, sc.z.mul_add(od.z, sd.z * od.w))),
                        sa.w.mul_add(od.x, sb.w.mul_add(od.y, sc.w.mul_add(od.z, sd.w * od.w))),
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
    Mat4 => Rotor3, Bivec3, Vec4, Vec3, f32,
    Mat4x4 => Rotor3x4, Bivec3x4, Vec4x4, Vec3x4, f32x4,
    Mat4x8 => Rotor3x8, Bivec3x8, Vec4x8, Vec3x8, f32x8
);

#[cfg(feature = "f64")]
mat4s!(
    DMat4 => DRotor3, DBivec3, DVec4, DVec3, f64,
    DMat4x2 => DRotor3x2, DBivec3x2, DVec4x2, DVec3x2, f64x2,
    DMat4x4 => DRotor3x4, DBivec3x4, DVec4x4, DVec3x4, f64x4
);

macro_rules! impl_partialeq_mat4 {
    ($($mt:ident),+) => {
        $(impl PartialEq for $mt {
            fn eq(&self, other: &Self) -> bool {
                self.cols[0] == other.cols[0]
                    && self.cols[1] == other.cols[1]
                    && self.cols[2] == other.cols[2]
                    && self.cols[3] == other.cols[3]
            }
        })+
    };
}

impl_partialeq_mat4!(Mat4);

#[cfg(feature = "f64")]
impl_partialeq_mat4!(DMat4);

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::*;
    use std::f32::consts::PI;
    use std::f32::consts::FRAC_PI_2;

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
            let rotor = Rotor3::from_euler_angles(alpha, beta, gamma);
            let mat = rotor.into_matrix();
            let rotor2 = mat.into_rotor3();
            assert!(Vec3::unit_x().rotated_by(rotor).eq_eps(Vec3::unit_x().rotated_by(rotor2)));
            assert!(Vec3::unit_y().rotated_by(rotor).eq_eps(Vec3::unit_y().rotated_by(rotor2)));
            assert!(Vec3::unit_z().rotated_by(rotor).eq_eps(Vec3::unit_z().rotated_by(rotor2)));
        }

    }
}
