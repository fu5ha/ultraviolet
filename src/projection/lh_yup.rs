//! Projection matrices that are intended to be used when the base coordinate
//! system (i.e. the one used by the application code) is left-handed, with the
//! x-axis pointing right, y-axis pointing *up*, and z-axis pointing *into the
//! screen*.

use crate::mat::*;
use crate::vec::*;

/// Orthographic projection matrix for use with OpenGL.
///
/// This matrix is meant to be used when the source coordinate space is left-handed and y-up
/// and the destination space is left-handed
/// and y-up, with Z (depth) clip extending from -1.0 (close) to 1.0 (far).
#[inline]
pub fn orthographic_gl(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
    let rml = right - left;
    let rpl = right + left;
    let tmb = top - bottom;
    let tpb = top + bottom;
    let fmn = far - near;
    let fpn = far + near;
    Mat4::new(
        Vec4::new(2.0 / rml, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 2.0 / tmb, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 2.0 / fmn, 0.0),
        Vec4::new(-(rpl / rml), -(tpb / tmb), -(fpn / fmn), 1.0),
    )
}

/// Orthographic projection matrix for use with Vulkan.
///
/// This matrix is meant to be used when the source coordinate space is left-handed and y-up
/// and the destination space is right-handed
/// and y-down, with Z (depth) clip extending from 0.0 (close) to 1.0 (far).
#[inline]
pub fn orthographic_vk(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
    let rml = right - left;
    let rpl = right + left;
    let tmb = top - bottom;
    let tpb = top + bottom;
    let fmn = far - near;
    Mat4::new(
        Vec4::new(2.0 / rml, 0.0, 0.0, 0.0),
        Vec4::new(0.0, -2.0 / tmb, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 1.0 / fmn, 0.0),
        Vec4::new(-(rpl / rml), -(tpb / tmb), -(near / fmn), 1.0),
    )
}

/// Orthographic projection matrix for use with DirectX.
///
/// This matrix is meant to be used when the source coordinate space is left-handed and y-up
/// and the destination space is left-handed
/// and y-up, with Z (depth) clip extending from 0.0 (close) to 1.0 (far).
#[inline]
pub fn orthographic_dx(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
    let rml = right - left;
    let rpl = right + left;
    let tmb = top - bottom;
    let tpb = top + bottom;
    let fmn = far - near;
    Mat4::new(
        Vec4::new(2.0 / rml, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 2.0 / tmb, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 1.0 / fmn, 0.0),
        Vec4::new(-(rpl / rml), -(tpb / tmb), -(near / fmn), 1.0),
    )
}
