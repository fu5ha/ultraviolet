use crate::{IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4};
use encase::{
    matrix::{AsMutMatrixParts, AsRefMatrixParts, impl_matrix},
    vector::{AsMutVectorParts, AsRefVectorParts, impl_vector},
};

impl_vector!(2, Vec2, f32; using From);
impl_vector!(2, UVec2, u32; using From);
impl_vector!(2, IVec2, i32; using From);

impl_vector!(3, Vec3, f32; using From);
impl_vector!(3, UVec3, u32; using From);
impl_vector!(3, IVec3, i32; using From);

impl_vector!(4, Vec4, f32; using From);
impl_vector!(4, UVec4, u32; using From);
impl_vector!(4, IVec4, i32; using From);

impl_matrix!(2, 2, Mat2, f32; using From);
impl_matrix!(3, 3, Mat3, f32; using From);
impl_matrix!(4, 4, Mat4, f32; using From);

macro_rules! impl_vector_traits {
    ($n:literal, $type:ty, $el_ty:ty) => {
        impl AsRefVectorParts<$el_ty, $n> for $type {
            fn as_ref_parts(&self) -> &[$el_ty; $n] {
                self.as_slice().try_into().unwrap()
            }
        }
        impl AsMutVectorParts<$el_ty, $n> for $type {
            fn as_mut_parts(&mut self) -> &mut [$el_ty; $n] {
                self.as_mut_slice().try_into().unwrap()
            }
        }
    };
}

impl_vector_traits!(2, Vec2, f32);
impl_vector_traits!(2, UVec2, u32);
impl_vector_traits!(2, IVec2, i32);

impl_vector_traits!(3, Vec3, f32);
impl_vector_traits!(3, UVec3, u32);
impl_vector_traits!(3, IVec3, i32);

impl_vector_traits!(4, Vec4, f32);
impl_vector_traits!(4, UVec4, u32);
impl_vector_traits!(4, IVec4, i32);

macro_rules! impl_matrix_traits {
    ($c:literal, $r:literal, $type:ty, $el_ty:ty) => {
        impl AsRefMatrixParts<$el_ty, $c, $r> for $type {
            fn as_ref_parts(&self) -> &[[$el_ty; $r]; $c] {
                unsafe {
                    ::core::mem::transmute::<&[$el_ty; $r * $c], &[[$el_ty; $r]; $c]>(
                        self.as_array(),
                    )
                }
            }
        }
        impl AsMutMatrixParts<$el_ty, $c, $r> for $type {
            fn as_mut_parts(&mut self) -> &mut [[$el_ty; $r]; $c] {
                unsafe {
                    ::core::mem::transmute::<&mut [$el_ty; $r * $c], &mut [[$el_ty; $r]; $c]>(
                        self.as_mut_array(),
                    )
                }
            }
        }
    };
}

impl_matrix_traits!(2, 2, Mat2, f32);
impl_matrix_traits!(3, 3, Mat3, f32);
impl_matrix_traits!(4, 4, Mat4, f32);
