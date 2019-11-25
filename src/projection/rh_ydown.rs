//! Projection matrices that are intended to be used when the base coordinate
//! system (i.e. the one used by the application code) is right-handed, with the
//! x-axis pointing right, y-axis pointing *down*, and z-axis pointing *into the
//! screen*.

use crate::mat::*;
use crate::vec::*;

/// Orthographic projection matrix meant to be used to convert a right-handed, y-down
/// coordinate space to be used with OpenGL,
/// which has a left-handed, y-up coordinate space where Z (depth) clip goes from -1.0 (close) to 1.0 (far).
#[inline]
pub fn orthographic_gl(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
    let rml = right - left;
    let rpl = right + left;
    let tmb = top - bottom;
    let tpb = top + bottom;
    let fmn = far - near;
    let fpn = far + near;
    Mat4::new(
        Vec4::new(2.0 / rml, 0.0, 0.0, -(rpl / rml)),
        Vec4::new(0.0, -2.0 / tmb, 0.0, -(tpb / tmb)),
        Vec4::new(0.0, 0.0, -2.0 / fmn, -(fpn / fmn)),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    )
}

/// Orthographic projection matrix meant to be used to convert a right-handed, y-down
/// coordinate space to be used with Vulkan,
/// which has a right-handed, y-down coordinate space where Z (depth) clip goes from 0.0 (close) to 1.0 (far).
#[inline]
pub fn orthographic_vk(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
    let rml = right - left;
    let rpl = right + left;
    let tmb = top - bottom;
    let tpb = top + bottom;
    let nmf = near - far;
    Mat4::new(
        Vec4::new(2.0 / rml, 0.0, 0.0, -(rpl / rml)),
        Vec4::new(0.0, 2.0 / tmb, 0.0, -(tpb / tmb)),
        Vec4::new(0.0, 0.0, -1.0 / nmf, -(near / nmf)),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    )
}

/// Orthographic projection matrix meant to be used to convert a right-handed, y-down
/// coordinate space to be used with DirectX,
/// which has a left-handed, y-up coordinate space where Z (depth) clip goes from 0.0 (close) to 1.0 (far).
#[inline]
pub fn orthographic_dx(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
    let rml = right - left;
    let rpl = right + left;
    let tmb = top - bottom;
    let tpb = top + bottom;
    let nmf = near - far;
    Mat4::new(
        Vec4::new(2.0 / rml, 0.0, 0.0, -(rpl / rml)),
        Vec4::new(0.0, -2.0 / tmb, 0.0, -(tpb / tmb)),
        Vec4::new(0.0, 0.0, -1.0 / nmf, -(near / nmf)),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    )
}
