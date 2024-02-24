//! Projection matrices that are intended to be used when the base coordinate
//! system (i.e. the one used by the application code) is assumes +X points right, +Y points up,
//! and +Z points towards the viewer, similar to Godot, Maya, Houdini, Substance, etc.:
//!
//! ```ignore,log
//!     +y
//!     |
//!     |
//!     0----- +x
//!    /
//!   /
//! +z
//! ```
//!
//! This is reexported at the root of `projections` as it is the
//! de-facto standard coordinate system for doing computer graphics programming.

use crate::mat::*;
use crate::vec::*;

/// Orthographic projection matrix for use with OpenGL.
///
/// This matrix is meant to be used when the source coordinate space is right-handed and y-up
/// (the standard computer graphics coordinate space)and the destination space is left-handed
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
        Vec4::new(0.0, 0.0, -2.0 / fmn, 0.0),
        Vec4::new(-(rpl / rml), -(tpb / tmb), -(fpn / fmn), 1.0),
    )
}

/// Orthographic projection matrix for use with Vulkan.
///
/// This matrix is meant to be used when the source coordinate space is right-handed and y-up
/// (the standard computer graphics coordinate space)and the destination space is right-handed
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
        Vec4::new(0.0, 0.0, -1.0 / fmn, 0.0),
        Vec4::new(-(rpl / rml), -(tpb / tmb), -(near / fmn), 1.0),
    )
}

/// Orthographic projection matrix for use with WebGPU or DirectX.
///
/// This matrix is meant to be used when the source coordinate space is right-handed and y-up
/// (the standard computer graphics coordinate space)and the destination space is left-handed
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
        Vec4::new(0.0, 2.0 / tmb, 0.0, 0.0),
        Vec4::new(0.0, 0.0, -1.0 / fmn, 0.0),
        Vec4::new(-(rpl / rml), -(tpb / tmb), -(near / fmn), 1.0),
    )
}

/// Perspective projection matrix meant to be used with OpenGL.
///
/// * `vertical_fov` should be provided in radians.
/// * `aspect_ratio` should be the quotient `width / height`.
///
/// This matrix is meant to be used when the source coordinate space is right-handed and y-up
/// (the standard computer graphics coordinate space) and the destination coordinate space is
/// left-handed and y-up with Z (depth) clip extending from -1.0 (close) to 1.0 (far).
#[inline]
pub fn perspective_gl(vertical_fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Mat4 {
    let t = (vertical_fov / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;
    let nmf = z_near - z_far;

    Mat4::new(
        Vec4::new(sx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, sy, 0.0, 0.0),
        Vec4::new(0.0, 0.0, (z_far + z_near) / nmf, -1.0),
        Vec4::new(0.0, 0.0, 2.0 * z_near * z_far / nmf, 0.0),
    )
}

/// Perspective projection matrix meant to be used with WebGPU or DirectX.
///
/// * `vertical_fov` should be provided in radians.
/// * `aspect_ratio` should be the quotient `width / height`.
///
/// This matrix is meant to be used when the source coordinate space is right-handed and y-up
/// (the standard computer graphics coordinate space) and the destination coordinate space is
/// left-handed and y-up with Z (depth) clip extending from 0.0 (close) to 1.0 (far).
#[inline]
pub fn perspective_wgpu_dx(vertical_fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Mat4 {
    let t = (vertical_fov / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;
    let nmf = z_near - z_far;

    Mat4::new(
        Vec4::new(sx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, sy, 0.0, 0.0),
        Vec4::new(0.0, 0.0, z_far / nmf, -1.0),
        Vec4::new(0.0, 0.0, z_near * z_far / nmf, 0.0),
    )
}

/// Perspective projection matrix meant to be used with Vulkan.
///
/// * `vertical_fov` should be provided in radians.
/// * `aspect_ratio` should be the quotient `width / height`.
///
/// This matrix is meant to be used when the source coordinate space is right-handed and y-up
/// (the standard computer graphics coordinate space) and the destination coordinate space is
/// right-handed and y-down with Z (depth) clip extending from 0.0 (close) to 1.0 (far).
#[inline]
pub fn perspective_vk(vertical_fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Mat4 {
    let t = (vertical_fov / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;
    let nmf = z_near - z_far;

    Mat4::new(
        Vec4::new(sx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, -sy, 0.0, 0.0),
        Vec4::new(0.0, 0.0, z_far / nmf, -1.0),
        Vec4::new(0.0, 0.0, z_near * z_far / nmf, 0.0),
    )
}

/// Perspective projection matrix with infinite z-far plane meant to be used with OpenGL.
///
/// This is useful for extremely large scenes where having a far clip plane is extraneous anyway,
/// as allowing it to approach infinity it eliminates several approximate numerical computations
/// and so can improve z-fighting behavior.
///
/// * `vertical_fov` should be provided in radians.
/// * `aspect_ratio` should be the quotient `width / height`.
///
/// This matrix is meant to be used when the source coordinate space is right-handed and y-up
/// (the standard computer graphics coordinate space) and the destination coordinate space is
/// left-handed and y-up with Z (depth) clip extending from -1.0 (close) to 1.0 (far).
#[inline]
pub fn perspective_infinite_z_gl(vertical_fov: f32, aspect_ratio: f32, z_near: f32) -> Mat4 {
    let t = (vertical_fov / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;

    Mat4::new(
        Vec4::new(sx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, sy, 0.0, 0.0),
        Vec4::new(0.0, 0.0, -1.0, -1.0),
        Vec4::new(0.0, 0.0, -2.0 * z_near, 0.0),
    )
}

/// Perspective projection matrix with infinite z-far plane meant to be used with Vulkan.
///
/// This is useful for extremely large scenes where having a far clip plane is extraneous anyway,
/// as allowing it to approach infinity it eliminates several approximate numerical computations
/// and so can improve z-fighting behavior.
///
/// * `vertical_fov` should be provided in radians.
/// * `aspect_ratio` should be the quotient `width / height`.
///
/// This matrix is meant to be used when the source coordinate space is right-handed and y-up
/// (the standard computer graphics coordinate space) and the destination coordinate space is
/// right-handed and y-down with Z (depth) clip extending from 0.0 (close) to 1.0 (far).
#[inline]
pub fn perspective_infinite_z_vk(vertical_fov: f32, aspect_ratio: f32, z_near: f32) -> Mat4 {
    let t = (vertical_fov / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;

    Mat4::new(
        Vec4::new(sx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, -sy, 0.0, 0.0),
        Vec4::new(0.0, 0.0, -1.0, -1.0),
        Vec4::new(0.0, 0.0, -z_near, 0.0),
    )
}

/// Perspective projection matrix with infinite z-far plane meant to be used with WebGPU or DirectX.
///
/// This is useful for extremely large scenes where having a far clip plane is extraneous anyway,
/// as allowing it to approach infinity it eliminates several approximate numerical computations
/// and so can improve z-fighting behavior.
///
/// * `vertical_fov` should be provided in radians.
/// * `aspect_ratio` should be the quotient `width / height`.
///
/// This matrix is meant to be used when the source coordinate space is right-handed and y-up
/// (the standard computer graphics coordinate space) and the destination coordinate space is
/// left-handed and y-up with Z (depth) clip extending from 0.0 (close) to 1.0 (far).
#[inline]
pub fn perspective_infinite_z_wgpu_dx(vertical_fov: f32, aspect_ratio: f32, z_near: f32) -> Mat4 {
    let t = (vertical_fov / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;

    Mat4::new(
        Vec4::new(sx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, sy, 0.0, 0.0),
        Vec4::new(0.0, 0.0, -1.0, -1.0),
        Vec4::new(0.0, 0.0, -z_near, 0.0),
    )
}

/// Perspective projection matrix with reversed z-axis meant to be used with WebGPU, DirectX, or OpenGL.
///
/// Reversed-Z provides significantly better precision and therefore reduced z-fighting
/// for most depth situations, especially when a floating-point depth buffer is used. You'll want to use
/// a reversed depth comparison function and depth clear value when using this projection.
///
/// * `vertical_fov` should be provided in radians.
/// * `aspect_ratio` should be the quotient `width / height`.
///
/// This matrix is meant to be used when the source coordinate space is right-handed and y-up
/// (the standard computer graphics coordinate space) and the destination coordinate space is
/// left-handed and y-up with Z (depth) clip extending from 0.0 (close) to 1.0 (far).
///
/// **Note that in order for this to work properly with OpenGL, you'll need to use the `gl_arb_clip_control` extension
/// and set the z clip from 0.0 to 1.0 rather than the default -1.0 to 1.0**
#[inline]
pub fn perspective_reversed_z_wgpu_dx_gl(
    vertical_fov: f32,
    aspect_ratio: f32,
    z_near: f32,
    z_far: f32,
) -> Mat4 {
    let t = (vertical_fov / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;
    let nmf = z_near - z_far;

    Mat4::new(
        Vec4::new(sx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, sy, 0.0, 0.0),
        Vec4::new(0.0, 0.0, -z_far / nmf - 1.0, -1.0),
        Vec4::new(0.0, 0.0, -z_near * z_far / nmf, 0.0),
    )
}

/// Perspective projection matrix with reversed z-axis meant to be used with Vulkan.
///
/// Reversed-Z provides significantly better precision and therefore reduced z-fighting
/// for most depth situations, especially when a floating-point depth buffer is used. You'll want to use
/// a reversed depth comparison function and depth clear value when using this projection.
///
/// * `vertical_fov` should be provided in radians.
/// * `aspect_ratio` should be the quotient `width / height`.
///
/// This matrix is meant to be used when the source coordinate space is right-handed and y-up
/// (the standard computer graphics coordinate space) and the destination coordinate space is
/// right-handed and y-down with Z (depth) clip extending from 0.0 (close) to 1.0 (far).
#[inline]
pub fn perspective_reversed_z_vk(
    vertical_fov: f32,
    aspect_ratio: f32,
    z_near: f32,
    z_far: f32,
) -> Mat4 {
    let t = (vertical_fov / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;
    let nmf = z_near - z_far;

    Mat4::new(
        Vec4::new(sx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, -sy, 0.0, 0.0),
        Vec4::new(0.0, 0.0, -z_near / nmf, -1.0),
        Vec4::new(0.0, 0.0, -z_near * z_far / nmf, 0.0),
    )
}

/// Perspective projection matrix with reversed and infinite z-axis meant to be used with WebGPU, OpenGL, or DirectX.
///
/// Reversed-Z provides significantly better precision and therefore reduced z-fighting
/// for most depth situations, especially when a floating-point depth buffer is used. You'll want to use
/// a reversed depth comparison function and depth clear value when using this projection.
///
/// Infinte-Z is useful for extremely large scenes where having a far clip plane is extraneous anyway,
/// as allowing it to approach infinity it eliminates several approximate numerical computations
/// and so can improve z-fighting behavior.
///
/// Combining them gives the best of both worlds for large scenes.
///
/// * `vertical_fov` should be provided in radians.
/// * `aspect_ratio` should be the quotient `width / height`.
///
/// This matrix is meant to be used when the source coordinate space is right-handed and y-up
/// (the standard computer graphics coordinate space) and the destination coordinate space is
/// left-handed and y-up with Z (depth) clip extending from 0.0 (close) to 1.0 (far).
///
/// **Note that in order for this to work properly with OpenGL, you'll need to use the `gl_arb_clip_control` extension
/// and set the z clip from 0.0 to 1.0 rather than the default -1.0 to 1.0**
#[inline]
pub fn perspective_reversed_infinite_z_wgpu_dx_gl(
    vertical_fov: f32,
    aspect_ratio: f32,
    z_near: f32,
) -> Mat4 {
    let t = (vertical_fov / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;

    Mat4::new(
        Vec4::new(sx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, sy, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, -1.0),
        Vec4::new(0.0, 0.0, z_near, 0.0),
    )
}

/// Perspective projection matrix with reversed and infinite z-axis meant to be used with Vulkan.
///
/// Reversed-Z provides significantly better precision and therefore reduced z-fighting
/// for most depth situations, especially when a floating-point depth buffer is used. You'll want to use
/// a reversed depth comparison function and depth clear value when using this projection.
///
/// Infinte-Z is useful for extremely large scenes where having a far clip plane is extraneous anyway,
/// as allowing it to approach infinity it eliminates several approximate numerical computations
/// and so can improve z-fighting behavior.
///
/// Combining them gives the best of both worlds for large scenes.
///
/// * `vertical_fov` should be provided in radians.
/// * `aspect_ratio` should be the quotient `width / height`.
///
/// This matrix is meant to be used when the source coordinate space is right-handed and y-up
/// (the standard computer graphics coordinate space) and the destination coordinate space is
/// right-handed and y-down with Z (depth) clip extending from 0.0 (close) to 1.0 (far).
#[inline]
pub fn perspective_reversed_infinite_z_vk(
    vertical_fov: f32,
    aspect_ratio: f32,
    z_near: f32,
) -> Mat4 {
    let t = (vertical_fov / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;

    Mat4::new(
        Vec4::new(sx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, -sy, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, -1.0),
        Vec4::new(0.0, 0.0, z_near, 0.0),
    )
}
