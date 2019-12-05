//! Square matrices.
use std::ops::*;

use crate::*;

use wide::f32x4;

macro_rules! mat2s {
    ($($n:ident, $vt:ident => $t:ident),+) => {
        /// A 2x2 square matrix.
        ///
        /// Useful for performing linear transformations (rotation, scaling) on 2d vectors.
        $(#[derive(Clone, Copy, Debug)]
        #[repr(C)]
        pub struct $n {
            pub cols: [$vt; 2],
        }

        impl $n {
            #[inline]
            pub fn new(col1: $vt, col2: $vt) -> Self {
                $n {
                    cols: [col1, col2],
                }
            }

            #[inline]
            pub fn layout() -> alloc::alloc::Layout {
                alloc::alloc::Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<$vt>()).unwrap()
            }

            #[inline]
            pub fn as_slice(&self) -> &[$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(std::mem::transmute(self), 4)
                }
            }

            #[inline]
            pub fn as_component_slice(&self) -> &[$vt] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(std::mem::transmute(self), 2)
                }
            }

            #[inline]
            pub fn as_byte_slice(&self) -> &[u8] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(std::mem::transmute(self), 4 * std::mem::size_of::<$t>())
                }
            }

            #[inline]
            pub fn as_mut_slice(&mut self) -> &mut [$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(std::mem::transmute(self), 4)
                }
            }

            #[inline]
            pub fn as_mut_component_slice(&mut self) -> &mut [$vt] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(std::mem::transmute(self), 2)
                }
            }

            #[inline]
            pub fn as_mut_byte_slice(&mut self) -> &mut [u8] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(std::mem::transmute(self), 4 * std::mem::size_of::<$t>())
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
                    $vt::new(
                        sa.x * oa.x + sb.x * oa.y,
                        sa.x * ob.x + sb.x * ob.y,
                    ),
                    $vt::new(
                        sa.y * oa.x + sb.y * oa.y,
                        sa.y * ob.x + sb.y * ob.y,
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
                    a.x * rhs.x + b.x * rhs.y,
                    a.y * rhs.x + b.y * rhs.y,
                )
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

        )+
    }
}

mat2s!(Mat2, Vec2 => f32 , Wat2, Wec2 => f32x4);

macro_rules! mat3s {
    ($($n:ident => $rt:ident, $bt:ident, $m4t:ident, $v4t:ident, $vt:ident, $t:ident),+) => {
        /// A 3x3 square matrix.
        ///
        /// Useful for performing linear transformations (rotation, scaling) on 3d vectors,
        /// or for performing arbitrary transformations (linear + translation, projection, etc)
        /// on homogeneous 2d vectors
        $(#[derive(Clone, Copy, Debug)]
        #[repr(C)]
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
            pub fn from_scale(scale: $t) -> Self {
                let zero = $t::from(0.0);
                Self::new(
                    $vt::new(scale, zero, zero),
                    $vt::new(zero, scale, zero),
                    $vt::new(zero, zero, scale),
                )
            }

            #[inline]
            pub fn from_nonuniform_scale(scale: $vt) -> Self {
                let zero = $t::from(0.0);
                Self::new(
                    $vt::new(scale.x, zero, zero),
                    $vt::new(zero, scale.y, zero),
                    $vt::new(zero, zero, scale.z),
                )
            }

            #[inline]
            pub fn identity() -> Self {
                Self::new(
                    $vt::new($t::from(1.0), $t::from(0.0), $t::from(0.0)),
                    $vt::new($t::from(0.0), $t::from(1.0), $t::from(0.0)),
                    $vt::new($t::from(0.0), $t::from(0.0), $t::from(1.0)))
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
                let (sr, cr) = roll.sin_cos();
                let (sp, cp) = pitch.sin_cos();
                let (sy, cy) = yaw.sin_cos();

                Self::new(
                    $vt::new(cy * cp, cy * sp * sr - sy * cr, cy * sp * cr + sy * sr),
                    $vt::new(sy * cp, sy * sp * sr + cy * cr, sy * sp * cr - cy * sr),
                    $vt::new(-sp, cp * sr, cp * cr))
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
                Self::from_euler_angles($t::from(0.0), angle, $t::from(0.0))
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
                Self::from_euler_angles(angle, $t::from(0.0), $t::from(0.0))
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
                Self::from_euler_angles($t::from(0.0), $t::from(0.0), angle)
            }

            /// Construct a rotation matrix given a bivector which defines a plane, rotation orientation,
            /// and rotation angle. The bivector defines the plane and orientation, and its magnitude
            /// defines the angle of rotation in radians.
            ///
            /// This is the equivalent of an axis-angle rotation.
            #[inline]
            pub fn from_angle_plane(planeangle: $bt) -> Self {
                $rt::from_angle_plane(planeangle).into_matrix()
            }

            #[inline]
            pub fn into_homogeneous(self) -> $m4t {
                let zero = $t::from(0.0);
                let one = $t::from(1.0);
                $m4t::new(
                    self.cols[0].into(),
                    self.cols[1].into(),
                    self.cols[2].into(),
                    $v4t::new(zero, zero, zero, one)
                )
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
                    std::slice::from_raw_parts(std::mem::transmute(self), 9)
                }
            }

            #[inline]
            pub fn as_component_slice(&self) -> &[$vt] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(std::mem::transmute(self), 3)
                }
            }

            #[inline]
            pub fn as_byte_slice(&self) -> &[u8] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(std::mem::transmute(self), 9 * std::mem::size_of::<$t>())
                }
            }

            #[inline]
            pub fn as_mut_slice(&mut self) -> &mut [$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(std::mem::transmute(self), 9)
                }
            }

            #[inline]
            pub fn as_mut_component_slice(&mut self) -> &mut [$vt] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(std::mem::transmute(self), 3)
                }
            }

            #[inline]
            pub fn as_mut_byte_slice(&mut self) -> &mut [u8] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(std::mem::transmute(self), 9 * std::mem::size_of::<$t>())
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
                unsafe {
                    std::mem::transmute(self)
                }
            }

            /// Returns a mutable unsafe pointer to the underlying data in the underlying type.
            /// This function is safe because all types here are repr(C) and can be represented
            /// as their underlying type.
            ///
            /// # Safety
            ///
            /// It is up to the caller to correctly use this pointer and its bounds.
            #[inline]
            pub fn as_mut_ptr(&self) -> *mut $t {
                unsafe {
                    std::mem::transmute(self)
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

        impl From<&[$t; 9]> for $n {
            #[inline]
            fn from(comps: &[$t; 9]) -> Self {
                Self::from(*comps)
            }
        }

        impl From<&mut [$t; 9]> for $n {
            #[inline]
            fn from(comps: &mut [$t; 9]) -> Self {
                Self::from(*comps)
            }
        }
        )+
    }
}

mat3s!(Mat3 => Rotor3, Bivec3, Mat4, Vec4, Vec3, f32, Wat3 => WRotor3, WBivec3, Wat4, Wec4, Wec3, f32x4);

macro_rules! mat4s {
    ($($n:ident => $rt:ident, $bt:ident, $vt:ident, $v3t:ident, $t:ident),+) => {
        /// A 4x4 square matrix.
        ///
        /// Useful for performing linear transformations (rotation, scaling) on 4d vectors,
        /// or for performing arbitrary transformations (linear + translation, projection, etc)
        /// on homogeneous 3d vectors.
        ///
        /// Note that most constructors assume that the matrix will be used as a homogeneous 3d
        /// transformation matrix.
        $(#[derive(Clone, Copy, Debug)]
        #[repr(C)]
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

            /// Assumes homogeneous 3d coordinates.
            #[inline]
            pub fn from_translation(trans: $v3t) -> Self {
                Self::new(
                    $vt::new($t::from(1.0), $t::from(0.0), $t::from(0.0), trans.x),
                    $vt::new($t::from(0.0), $t::from(1.0), $t::from(0.0), trans.y),
                    $vt::new($t::from(0.0), $t::from(0.0), $t::from(1.0), trans.z),
                    $vt::new($t::from(0.0), $t::from(0.0), $t::from(0.0), $t::from(1.0)))
            }

            /// Assumes homogeneous 3d coordinates.
            #[inline]
            pub fn from_scale(scale: $t) -> Self {
                let zero = $t::from(0.0);
                Self::new(
                    $vt::new(scale, zero, zero, zero),
                    $vt::new(zero, scale, zero, zero),
                    $vt::new(zero, zero, scale, zero),
                    $vt::new(zero, zero, zero, $t::from(1.0)),
                )
            }

            /// Assumes homogeneous 3d coordinates.
            #[inline]
            pub fn from_nonuniform_scale(scale: $vt) -> Self {
                let zero = $t::from(0.0);
                Self::new(
                    $vt::new(scale.x, zero, zero, zero),
                    $vt::new(zero, scale.y, zero, zero),
                    $vt::new(zero, zero, scale.z, zero),
                    $vt::new(zero, zero, zero, $t::from(1.0)),
                )
            }

            /// Angles are applied in the order roll -> pitch -> yaw
            ///
            /// - Yaw is rotation inside the xz plane ("around the y axis")
            /// - Pitch is rotation inside the yz plane ("around the x axis")
            /// - Roll is rotation inside the xy plane ("around the z axis")
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
                let (sr, cr) = roll.sin_cos();
                let (sp, cp) = pitch.sin_cos();
                let (sy, cy) = yaw.sin_cos();

                Self::new(
                    $vt::new(cy * cp, cy * sp * sr + sy * cr, cy * sp * cr + sy * sr, $t::from(0.0)),
                    $vt::new(-sy * cp, sy * sp * sr + cy * cr, sy * sp * cr - cy * sr, $t::from(0.0)),
                    $vt::new(-sp, cp * sr, cp * cr, $t::from(0.0)),
                    $vt::new($t::from(0.0), $t::from(0.0), $t::from(0.0), $t::from(1.0)))
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
                Self::from_euler_angles($t::from(0.0), angle, $t::from(0.0))
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
                // TODO: Easy optimization target.
                Self::from_euler_angles(angle, $t::from(0.0), $t::from(0.0))
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
                Self::from_euler_angles($t::from(0.0), $t::from(0.0), angle)
            }

            /// Construct a rotation matrix given a bivector which defines a plane, rotation orientation,
            /// and rotation angle. The bivector defines the plane and orientation, and its magnitude
            /// defines the angle of rotation in radians.
            ///
            /// This is the equivalent of an axis-angle rotation.
            ///
            /// Assumes homogeneous 3d coordinates.
            #[inline]
            pub fn from_angle_plane(planeangle: $bt) -> Self {
                $rt::from_angle_plane(planeangle).into_matrix().into_homogeneous()
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
                    std::slice::from_raw_parts(std::mem::transmute(self), 16)
                }
            }

            #[inline]
            pub fn as_component_slice(&self) -> &[$vt] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(std::mem::transmute(self), 4)
                }
            }

            #[inline]
            pub fn as_byte_slice(&self) -> &[u8] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts(std::mem::transmute(self), 16 * std::mem::size_of::<$t>())
                }
            }

            #[inline]
            pub fn as_mut_slice(&mut self) -> &mut [$t] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(std::mem::transmute(self), 16)
                }
            }

            #[inline]
            pub fn as_mut_component_slice(&mut self) -> &mut [$vt] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(std::mem::transmute(self), 4)
                }
            }

            #[inline]
            pub fn as_mut_byte_slice(&mut self) -> &mut [u8] {
                // This is safe because we are statically bounding our slices to the size of these
                // vectors
                unsafe {
                    std::slice::from_raw_parts_mut(std::mem::transmute(self), 16 * std::mem::size_of::<$t>())
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
                unsafe {
                    std::mem::transmute(self)
                }
            }

            /// Returns a mutable unsafe pointer to the underlying data in the underlying type.
            /// This function is safe because all types here are repr(C) and can be represented
            /// as their underlying type.
            ///
            /// # Safety
            ///
            /// It is up to the caller to correctly use this pointer and its bounds.
            #[inline]
            pub fn as_mut_ptr(&self) -> *mut $t {
                unsafe {
                    std::mem::transmute(self)
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

        impl From<&[$t; 16]> for $n {
            #[inline]
            fn from(comps: &[$t; 16]) -> Self {
                Self::from(*comps)
            }
        }

        impl From<&mut [$t; 16]> for $n {
            #[inline]
            fn from(comps: &mut [$t; 16]) -> Self {
                Self::from(*comps)
            }
        }

        )+
    }
}

mat4s!(Mat4 => Rotor3, Bivec3, Vec4, Vec3, f32, Wat4 => WRotor3, WBivec3, Wec4, Wec3, f32x4);
