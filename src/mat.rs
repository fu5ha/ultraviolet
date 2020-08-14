//! Square matrices.
use std::ops::*;

use crate::*;

use wide::f32x4;

#[cfg(feature = "serde")]
use serde::{de::SeqAccess, Deserialize, Deserializer, Serialize, Serializer};

macro_rules! mat2s {
    ($($n:ident => $m3t:ident, $v3t:ident, $vt:ident, $t:ident),+) => {
        /// A 2x2 square matrix.
        ///
        /// Useful for performing linear transformations (rotation, scaling) on 2d vectors.
        $(#[derive(Clone, Copy, Debug)]
        #[repr(C)]
        pub struct $n {
            pub cols: [$vt; 2],
        }

        derive_default_identity!($n);

        impl $n {
            #[inline]
            pub fn new(col1: $vt, col2: $vt) -> Self {
                $n {
                    cols: [col1, col2],
                }
            }

            #[inline]
            pub fn identity() -> Self {
                Self::new(
                    $vt::new($t::from(1.0), $t::from(0.0)),
                    $vt::new($t::from(0.0), $t::from(1.0)),
                )
            }

            /// Turn this into a homogeneous 2d transformation matrix.
            #[inline]
            pub fn into_homogeneous(self) -> $m3t {
                $m3t::new(
                    self.cols[0].into(),
                    self.cols[1].into(),
                    $v3t::new($t::from(0.0), $t::from(0.0), $t::from(1.0))
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

mat2s!(Mat2 => Mat3, Vec3, Vec2, f32, Wat2 => Wat3, Wec3, Wec2, f32x4);
mat2s!(Mat2d => Mat3d, Vec3d, Vec2d, f64);

impl PartialEq for Mat2 {
    fn eq(&self, other: &Self) -> bool {
        self.cols[0] == other.cols[0] && self.cols[1] == other.cols[1]
    }

    fn ne(&self, other: &Self) -> bool {
        self.cols[0] != other.cols[0] || self.cols[1] != other.cols[1]
    }
}

#[cfg(feature = "serde")]
impl Serialize for Mat2 {
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(4))?;

        seq.serialize_element(&self.cols[0].x)?;
        seq.serialize_element(&self.cols[0].y)?;
        seq.serialize_element(&self.cols[1].x)?;
        seq.serialize_element(&self.cols[1].y)?;
        seq.end()
    }
}

#[cfg(feature = "serde")]
struct Mat2Visitor {}

#[cfg(feature = "serde")]
impl Mat2Visitor {
    pub fn new() -> Self {
        Mat2Visitor {}
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for Mat2Visitor {
    type Value = Mat2;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("array of 4 floats")
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        use serde::de::Error;

        Ok(Self::Value {
            cols: [
                Vec2::new(
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(0, &self)),
                    },
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(1, &self)),
                    },
                ),
                Vec2::new(
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(2, &self)),
                    },
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(3, &self)),
                    },
                ),
            ],
        })
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Mat2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(4, Mat2Visitor::new())
    }

    //    @TODO I understand how to implement it in the context of arrays but not matrices
    //    fn deserialize_in_place<D>(
    //        deserializer: D,
    //        place: &mut Self,
    //    ) -> Result<(), <D as Deserializer<'de>>::Error>
    //    where
    //        D: Deserializer<'de>,
    //    {
    //        unimplemented!()
    //    }
}

macro_rules! mat3s {
    ($($n:ident => $rt:ident, $bt:ident, $m4t:ident, $v4t:ident, $v2t:ident, $vt:ident, $t:ident),+) => {
        /// A 3x3 square matrix.
        ///
        /// Useful for performing linear transformations (rotation, scaling) on 3d vectors,
        /// or for performing arbitrary transformations (linear +   translation, projection, etc)
        /// on homogeneous 2d vectors
        $(#[derive(Clone, Copy, Debug)]
        #[repr(C)]
        pub struct $n {
            pub cols: [$vt; 3],
        }

        derive_default_identity!($n);

        impl $n {
            #[inline]
            pub fn new(col1: $vt, col2: $vt, col3: $vt) -> Self {
                $n {
                    cols: [col1, col2, col3],
                }
            }

            /// Assumes homogeneous 2d coordinates.
            #[inline]
            pub fn from_translation(trans: $v2t) -> Self {
                Self::new(
                    $vt::new($t::from(1.0), $t::from(0.0), $t::from(0.0)),
                    $vt::new($t::from(0.0), $t::from(1.0), $t::from(0.0)),
                    $vt::new(trans.x, trans.y, $t::from(1.0)))
            }

            /// Assumes homogeneous 2d coordinates.
            #[inline]
            pub fn from_scale_homogeneous(scale: $t) -> Self {
                let zero = $t::from(0.0);
                Self::new(
                    $vt::new(scale, zero, zero),
                    $vt::new(zero, scale, zero),
                    $vt::new(zero, zero, $t::from(1.0)),
                )
            }

            /// Assumes homogeneous 2d coordinates.
            #[inline]
            pub fn from_nonuniform_scale_homogeneous(scale: $vt) -> Self {
                let zero = $t::from(0.0);
                Self::new(
                    $vt::new(scale.x, zero, zero),
                    $vt::new(zero, scale.y, zero),
                    $vt::new(zero, zero, $t::from(1.0)),
                )
            }

            /// Builds a homogeneous 2d rotation matrix (in the xy plane) from a given angle in radians.
            #[inline]
            pub fn from_rotation_homogeneous(angle: $t) -> Self {
                let (s, c) = angle.sin_cos();
                let zero = $t::from(0.0);
                Self::new(
                    $vt::new(c, s, zero),
                    $vt::new(s, -c, zero),
                    $vt::new(zero, zero, $t::from(1.0)),
                )
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
                Self::from_euler_angles($t::from(0.0), $t::from(0.0), angle)
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
                Self::from_euler_angles(angle, $t::from(0.0), $t::from(0.0))
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
                let zero = $t::from(0.0);
                let one = $t::from(1.0);
                $m4t::new(
                    self.cols[0].into(),
                    self.cols[1].into(),
                    self.cols[2].into(),
                    $v4t::new(zero, zero, zero, one)
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
                let det = self.cols[2].dot(y);
                let inv_det = $t::from(1.0) / det;

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
                        sa.y * oa.x + sb.y * oa.y + sc.y * oa.z,
                        sa.z * oa.x + sb.z * oa.y + sc.z * oa.z,
                    ),
                    $vt::new(
                        sa.x * ob.x + sb.x * ob.y + sc.x * ob.z,
                        sa.y * ob.x + sb.y * ob.y + sc.y * ob.z,
                        sa.z * ob.x + sb.z * ob.y + sc.z * ob.z,
                    ),
                    $vt::new(
                        sa.x * oc.x + sb.x * oc.y + sc.x * oc.z,
                        sa.y * oc.x + sb.y * oc.y + sc.y * oc.z,
                        sa.z * oc.x + sb.z * oc.y + sc.z * oc.z,
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

mat3s!(Mat3 => Rotor3, Bivec3, Mat4, Vec4, Vec2, Vec3, f32, Wat3 => WRotor3, WBivec3, Wat4, Wec4, Wec2, Wec3, f32x4);
mat3s!(Mat3d => Rotor3d, Bivec3d, Mat4d, Vec4d, Vec2d, Vec3d, f64);

impl PartialEq for Mat3 {
    fn eq(&self, other: &Self) -> bool {
        self.cols[0] == other.cols[0]
            && self.cols[1] == other.cols[1]
            && self.cols[2] == other.cols[2]
    }

    fn ne(&self, other: &Self) -> bool {
        self.cols[0] != other.cols[0]
            || self.cols[1] != other.cols[1]
            || self.cols[2] != other.cols[2]
    }
}

#[cfg(feature = "serde")]
impl Serialize for Mat3 {
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(9))?;

        seq.serialize_element(&self.cols[0].x)?;
        seq.serialize_element(&self.cols[0].y)?;
        seq.serialize_element(&self.cols[0].z)?;
        seq.serialize_element(&self.cols[1].x)?;
        seq.serialize_element(&self.cols[1].y)?;
        seq.serialize_element(&self.cols[1].z)?;
        seq.serialize_element(&self.cols[2].x)?;
        seq.serialize_element(&self.cols[2].y)?;
        seq.serialize_element(&self.cols[2].z)?;
        seq.end()
    }
}

#[cfg(feature = "serde")]
struct Mat3Visitor {}

#[cfg(feature = "serde")]
impl Mat3Visitor {
    pub fn new() -> Self {
        Mat3Visitor {}
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for Mat3Visitor {
    type Value = Mat3;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("array of 9 floats")
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        use serde::de::Error;

        Ok(Self::Value {
            cols: [
                Vec3::new(
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(0, &self)),
                    },
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(1, &self)),
                    },
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(2, &self)),
                    },
                ),
                Vec3::new(
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(3, &self)),
                    },
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(4, &self)),
                    },
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(5, &self)),
                    },
                ),
                Vec3::new(
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(6, &self)),
                    },
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(7, &self)),
                    },
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(8, &self)),
                    },
                ),
            ],
        })
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Mat3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(9, Mat3Visitor::new())
    }

    //    @TODO I understand how to implement it in the context of arrays but not matrices
    //    fn deserialize_in_place<D>(
    //        deserializer: D,
    //        place: &mut Self,
    //    ) -> Result<(), <D as Deserializer<'de>>::Error>
    //    where
    //        D: Deserializer<'de>,
    //    {
    //        unimplemented!()
    //    }
}

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

        derive_default_identity!($n);

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
                    $vt::new($t::from(1.0), $t::from(0.0), $t::from(0.0), $t::from(0.0)),
                    $vt::new($t::from(0.0), $t::from(1.0), $t::from(0.0), $t::from(0.0)),
                    $vt::new($t::from(0.0), $t::from(0.0), $t::from(1.0), $t::from(0.0)),
                    $vt::new(trans.x, trans.y, trans.z, $t::from(1.0)))
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
            pub fn from_nonuniform_scale(scale: $v3t) -> Self {
                let zero = $t::from(0.0);
                Self::new(
                    $vt::new(scale.x, zero, zero, zero),
                    $vt::new(zero, scale.y, zero, zero),
                    $vt::new(zero, zero, scale.z, zero),
                    $vt::new(zero, zero, zero, $t::from(1.0)),
                )
            }

            /// Full 4d diagonal matrix.
            #[inline]
            pub fn from_scale_4d(scale: $t) -> Self {
                let zero = $t::from(0.0);
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
                let zero = $t::from(0.0);
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
                Self::from_euler_angles($t::from(0.0), $t::from(0.0), angle)
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
                Self::from_euler_angles(angle, $t::from(0.0), $t::from(0.0))
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
                    $vt::new(r.x, u.x, -f.x, $t::from(0.0)),
                    $vt::new(r.y, u.y, -f.y, $t::from(0.0)),
                    $vt::new(r.z, u.z, -f.z, $t::from(0.0)),
                    $vt::new(-r.dot(eye), -u.dot(eye), f.dot(eye), $t::from(1.0))
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
                    $vt::new(r.x, u.x, f.x, $t::from(0.0)),
                    $vt::new(r.y, u.y, f.y, $t::from(0.0)),
                    $vt::new(r.z, u.z, f.z, $t::from(0.0)),
                    $vt::new(-r.dot(eye), -u.dot(eye), -f.dot(eye), $t::from(1.0))
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

            /// If this matrix is not currently invertable, this function will return
            /// an invalid inverse. This status is not checked by the library.
            #[inline]
            pub fn inversed(&self) -> Self {
                let (m00, m01, m02, m03) = self.cols[0].into();
                let (m10, m11, m12, m13) = self.cols[1].into();
                let (m20, m21, m22, m23) = self.cols[2].into();
                let (m30, m31, m32, m33) = self.cols[3].into();

                let coef00 = m22 * m33 - m32 * m23;
                let coef02 = m12 * m33 - m32 * m13;
                let coef03 = m12 * m23 - m22 * m13;

                let coef04 = m21 * m33 - m31 * m23;
                let coef06 = m11 * m33 - m31 * m13;
                let coef07 = m11 * m23 - m21 * m13;

                let coef08 = m21 * m32 - m31 * m22;
                let coef10 = m11 * m32 - m31 * m12;
                let coef11 = m11 * m22 - m21 * m12;

                let coef12 = m20 * m33 - m30 * m23;
                let coef14 = m10 * m33 - m30 * m13;
                let coef15 = m10 * m23 - m20 * m13;

                let coef16 = m20 * m32 - m30 * m22;
                let coef18 = m10 * m32 - m30 * m12;
                let coef19 = m10 * m22 - m20 * m12;

                let coef20 = m20 * m31 - m30 * m21;
                let coef22 = m10 * m31 - m30 * m11;
                let coef23 = m10 * m21 - m20 * m11;

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

                let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
                let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
                let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
                let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

                let sign_a = $vt::new($t::from(1.0), $t::from(-1.0), $t::from(1.0), $t::from(-1.0));
                let sign_b = $vt::new($t::from(-1.0), $t::from(1.0), $t::from(-1.0), $t::from(1.0));

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

                let rcp_det = $t::from(1.0) / dot1;
                inverse * rcp_det
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
                        sa.y * oa.x + sb.y * oa.y + sc.y * oa.z + sd.y * oa.w,
                        sa.z * oa.x + sb.z * oa.y + sc.z * oa.z + sd.z * oa.w,
                        sa.w * oa.x + sb.w * oa.y + sc.w * oa.z + sd.w * oa.w,
                    ),
                    $vt::new(
                        sa.x * ob.x + sb.x * ob.y + sc.x * ob.z + sd.x * ob.w,
                        sa.y * ob.x + sb.y * ob.y + sc.y * ob.z + sd.y * ob.w,
                        sa.z * ob.x + sb.z * ob.y + sc.z * ob.z + sd.z * ob.w,
                        sa.w * ob.x + sb.w * ob.y + sc.w * ob.z + sd.w * ob.w,
                    ),
                    $vt::new(
                        sa.x * oc.x + sb.x * oc.y + sc.x * oc.z + sd.x * oc.w,
                        sa.y * oc.x + sb.y * oc.y + sc.y * oc.z + sd.y * oc.w,
                        sa.z * oc.x + sb.z * oc.y + sc.z * oc.z + sd.z * oc.w,
                        sa.w * oc.x + sb.w * oc.y + sc.w * oc.z + sd.w * oc.w,
                    ),
                    $vt::new(
                        sa.x * od.x + sb.x * od.y + sc.x * od.z + sd.x * od.w,
                        sa.y * od.x + sb.y * od.y + sc.y * od.z + sd.y * od.w,
                        sa.z * od.x + sb.z * od.y + sc.z * od.z + sd.z * od.w,
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
                    a.w * rhs.x + b.w * rhs.y + c.w * rhs.z + d.w * rhs.w,
                )
            }
        }

        impl Mul<$t> for $n {
            type Output = Self;
            #[inline]
            fn mul(mut self, rhs: $t) -> Self {
                self.cols.iter_mut().for_each(|c| *c = rhs * *c);
                self
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

mat4s!(Mat4 => Rotor3, Bivec3, Vec4, Vec3, f32, Wat4 => WRotor3, WBivec3, Wec4, Wec3, f32x4);
mat4s!(Mat4d => Rotor3d, Bivec3d, Vec4d, Vec3d, f64);

impl PartialEq for Mat4 {
    fn eq(&self, other: &Self) -> bool {
        self.cols[0] == other.cols[0]
            && self.cols[1] == other.cols[1]
            && self.cols[2] == other.cols[2]
            && self.cols[3] == other.cols[3]
    }

    fn ne(&self, other: &Self) -> bool {
        self.cols[0] != other.cols[0]
            || self.cols[1] != other.cols[1]
            || self.cols[2] != other.cols[2]
            || self.cols[3] != other.cols[3]
    }
}

#[cfg(feature = "serde")]
impl Serialize for Mat4 {
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(16))?;

        seq.serialize_element(&self.cols[0].x)?;
        seq.serialize_element(&self.cols[0].y)?;
        seq.serialize_element(&self.cols[0].z)?;
        seq.serialize_element(&self.cols[0].w)?;
        seq.serialize_element(&self.cols[1].x)?;
        seq.serialize_element(&self.cols[1].y)?;
        seq.serialize_element(&self.cols[1].z)?;
        seq.serialize_element(&self.cols[1].w)?;
        seq.serialize_element(&self.cols[2].x)?;
        seq.serialize_element(&self.cols[2].y)?;
        seq.serialize_element(&self.cols[2].z)?;
        seq.serialize_element(&self.cols[2].w)?;
        seq.serialize_element(&self.cols[3].x)?;
        seq.serialize_element(&self.cols[3].y)?;
        seq.serialize_element(&self.cols[3].z)?;
        seq.serialize_element(&self.cols[3].w)?;
        seq.end()
    }
}

#[cfg(feature = "serde")]
struct Mat4Visitor {}

#[cfg(feature = "serde")]
impl Mat4Visitor {
    pub fn new() -> Self {
        Mat4Visitor {}
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for Mat4Visitor {
    type Value = Mat4;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("array of 16 floats")
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        use serde::de::Error;

        Ok(Self::Value {
            cols: [
                Vec4::new(
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(0, &self)),
                    },
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(1, &self)),
                    },
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(2, &self)),
                    },
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(3, &self)),
                    },
                ),
                Vec4::new(
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(4, &self)),
                    },
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(5, &self)),
                    },
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(6, &self)),
                    },
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(7, &self)),
                    },
                ),
                Vec4::new(
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(8, &self)),
                    },
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(9, &self)),
                    },
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(10, &self)),
                    },
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(11, &self)),
                    },
                ),
                Vec4::new(
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(12, &self)),
                    },
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(13, &self)),
                    },
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(14, &self)),
                    },
                    match seq.next_element::<f32>()? {
                        Some(val) => val,
                        None => return Err(Error::invalid_length(15, &self)),
                    },
                ),
            ],
        })
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Mat4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(16, Mat4Visitor::new())
    }

    //    @TODO I understand how to implement it in the context of arrays but not matrices
    //    fn deserialize_in_place<D>(
    //        deserializer: D,
    //        place: &mut Self,
    //    ) -> Result<(), <D as Deserializer<'de>>::Error>
    //    where
    //        D: Deserializer<'de>,
    //    {
    //        unimplemented!()
    //    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use crate::mat::{Mat2, Mat3, Mat4};
    use crate::vec::{Vec2, Vec3, Vec4};
    use serde_test::{assert_tokens, Token};

    #[test]
    fn mat2() {
        let mat2 = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0));

        assert_tokens(
            &mat2,
            &[
                Token::Seq { len: Some(4) },
                Token::F32(1.0),
                Token::F32(2.0),
                Token::F32(3.0),
                Token::F32(4.0),
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn mat3() {
        let mat3 = Mat3::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(4.0, 5.0, 6.0),
            Vec3::new(7.0, 8.0, 9.0),
        );

        assert_tokens(
            &mat3,
            &[
                Token::Seq { len: Some(9) },
                Token::F32(1.0),
                Token::F32(2.0),
                Token::F32(3.0),
                Token::F32(4.0),
                Token::F32(5.0),
                Token::F32(6.0),
                Token::F32(7.0),
                Token::F32(8.0),
                Token::F32(9.0),
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn mat4() {
        let mat4 = Mat4::new(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );

        assert_tokens(
            &mat4,
            &[
                Token::Seq { len: Some(16) },
                Token::F32(1.0),
                Token::F32(2.0),
                Token::F32(3.0),
                Token::F32(4.0),
                Token::F32(5.0),
                Token::F32(6.0),
                Token::F32(7.0),
                Token::F32(8.0),
                Token::F32(9.0),
                Token::F32(10.0),
                Token::F32(11.0),
                Token::F32(12.0),
                Token::F32(13.0),
                Token::F32(14.0),
                Token::F32(15.0),
                Token::F32(16.0),
                Token::SeqEnd,
            ],
        );
    }
}
