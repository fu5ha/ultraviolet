//! Projection matrices that are intended to be used when the base coordinate
//! system (i.e. the one used by the application code) has the x-axis pointing right,
//! y-axis pointing *down*, and z-axis pointing *towards the viewer*.
//!
//! Note that this module only exports orthographic matrices. That is because the only place that
//! a left-handed, y-down coordinate system is common is when thinking in primarily 2d.
//!
//! If you're using a left-handed, y-down source coordinate system for 3d...
//! stop it, get some help.

use crate::mat::*;
use crate::vec::*;

/// Orthographic projection matrix for use with OpenGL and a source "2d y-down" coordinate space.
///
/// This matrix is meant to be used when the source coordinate space is left-handed and y-down
/// (+X right, +Y down, +Z towards the viewer) and the destination space is left-handed
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
        Vec4::new(0.0, -2.0 / tmb, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 2.0 / fmn, 0.0),
        Vec4::new(-(rpl / rml), -(tpb / tmb), -(fpn / fmn), 1.0),
    )
}

/// Orthographic projection matrix for use with Vulkan and a source "2d y-down" coordinate space.
///
/// This matrix is meant to be used when the source coordinate space is left-handed and y-down
/// (+X right, +Y down, +Z towards the viewer) and the destination space is right-handed
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
        Vec4::new(0.0, 2.0 / tmb, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 1.0 / fmn, 0.0),
        Vec4::new(-(rpl / rml), -(tpb / tmb), -(near / fmn), 1.0),
    )
}

/// Orthographic projection matrix for use with WebGPU or DirectX.
///
/// This matrix is meant to be used when the source coordinate space is left-handed and y-down
/// (+X right, +Y down, +Z towards the viewer) and the destination space is left-handed
/// and y-up, with Z (depth) clip extending from 0.0 (close) to 1.0 (far).
#[inline]
pub fn orthographic_wgpu_dx(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
) -> Mat4 {
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

