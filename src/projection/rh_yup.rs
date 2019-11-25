//! Projection matrices that are intended to be used when the base coordinate
//! system (i.e. the one used by the application code) is right-handed with the
//! the x-axis pointing right, y-axis pointing *up*, and z-axis pointing *out of
//! the screen*. This is reexported at the root of `projections` as it is the
//! de-facto standard coordinate system for doing computer graphics programming.

use crate::mat::*;
use crate::vec::*;

/// Orthographic projection matrix meant to be used to convert a right-handed, y-up
/// coordinate space (the standard computer graphics coordinate space) to be used with OpenGL,
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
        Vec4::new(0.0, 2.0 / tmb, 0.0, -(tpb / tmb)),
        Vec4::new(0.0, 0.0, -2.0 / fmn, -(fpn / fmn)),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    )
}

/// Orthographic projection matrix meant to be used to convert a right-handed, y-up
/// coordinate space (the standard computer graphics coordinate space) to be used with Vulkan,
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
        Vec4::new(0.0, -2.0 / tmb, 0.0, -(tpb / tmb)),
        Vec4::new(0.0, 0.0, -1.0 / nmf, -(near / nmf)),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    )
}

/// Orthographic projection matrix meant to be used to convert a right-handed, y-up
/// coordinate space (the standard computer graphics coordinate space) to be used with DirectX,
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
        Vec4::new(0.0, 2.0 / tmb, 0.0, -(tpb / tmb)),
        Vec4::new(0.0, 0.0, -1.0 / nmf, -(near / nmf)),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    )
}

/// Perspective projection matrix meant to be used to convert a right-handed, y-up
/// coordinate space (the standard computer graphics coordinate space) to be used with OpenGL,
/// which has a left-handed, y-up coordinate space where Z (depth) clip goes from -1.0 (close) to 1.0 (far).
#[inline]
pub fn perspective_gl(vertical_fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Mat4 {
    let t = (vertical_fov / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;
    let nmf = z_near - z_far;

    Mat4::new(
        Vec4::new(sx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, sy, 0.0, 0.0),
        Vec4::new(0.0, 0.0, (z_far + z_near) / nmf, 2.0 * z_near * z_far / nmf),
        Vec4::new(0.0, 0.0, -1.0, 0.0),
    )
}

/// Perspective projection matrix meant to be used to convert a right-handed, y-up
/// coordinate space (the standard computer graphics coordinate space) to be used with DirectX,
/// which has a left-handed, y-up coordinate space where Z (depth) clip goes from 0.0 (close) to 1.0 (far).
#[inline]
pub fn perspective_dx(vertical_fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Mat4 {
    let t = (vertical_fov / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;
    let nmf = z_near - z_far;

    Mat4::new(
        Vec4::new(sx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, sy, 0.0, 0.0),
        Vec4::new(0.0, 0.0, z_far / nmf, z_near * z_far / nmf),
        Vec4::new(0.0, 0.0, -1.0, 0.0),
    )
}

/// Perspective projection matrix with infinite z-far plane meant to be used to convert a right-handed, y-up
/// coordinate space (the standard computer graphics coordinate space) to be used with OpenGL,
/// which has a left-handed, y-up coordinate space where Z (depth) clip goes from -1.0 (close) to 1.0 (far).
///
/// This is useful for extremely large scenes where having a far clip plane would not be useful as it eliminates
/// several approximate numerical computations and so can improve z-fighting behavior.
#[inline]
pub fn perspective_infinite_z_gl(vertical_fov: f32, aspect_ratio: f32, z_near: f32) -> Mat4 {
    let t = (vertical_fov / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;

    Mat4::new(
        Vec4::new(sx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, sy, 0.0, 0.0),
        Vec4::new(0.0, 0.0, -1.0, -2.0 * z_near),
        Vec4::new(0.0, 0.0, -1.0, 0.0),
    )
}

/// Perspective projection matrix with infinite z-far plane meant to be used to convert a right-handed, y-up
/// coordinate space (the standard computer graphics coordinate space) to be used with Vulkan,
/// which has a right-handed, y-down coordinate space where Z (depth) clip goes from 0.0 (close) to 1.0 (far).
///
/// This is useful for extremely large scenes where having a far clip plane would not be useful as it eliminates
/// several approximate numerical computations and so can improve z-fighting behavior.
#[inline]
pub fn perspective_infinite_z_vk(vertical_fov: f32, aspect_ratio: f32, z_near: f32) -> Mat4 {
    let t = (vertical_fov / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;

    Mat4::new(
        Vec4::new(sx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, -sy, 0.0, 0.0),
        Vec4::new(0.0, 0.0, -1.0, -z_near),
        Vec4::new(0.0, 0.0, -1.0, 0.0),
    )
}

/// Perspective projection matrix with infinite z-far plane meant to be used to convert a right-handed, y-up
/// coordinate space (the standard computer graphics coordinate space) to be used with DirectX,
/// which has a left-handed, y-up coordinate space where Z (depth) clip goes from 0.0 (close) to 1.0 (far).
///
/// This is useful for extremely large scenes where having a far clip plane would not be useful as it eliminates
/// several approximate numerical computations and so can improve z-fighting behavior.
#[inline]
pub fn perspective_infinite_z_dx(vertical_fov: f32, aspect_ratio: f32, z_near: f32) -> Mat4 {
    let t = (vertical_fov / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;

    Mat4::new(
        Vec4::new(sx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, sy, 0.0, 0.0),
        Vec4::new(0.0, 0.0, -1.0, -z_near),
        Vec4::new(0.0, 0.0, -1.0, 0.0),
    )
}

/// Perspective projection matrix meant to be used to convert a right-handed, y-up
/// coordinate space (the standard computer graphics coordinate space) to be used with Vulkan,
/// which has a right-handed, y-down coordinate space where Z (depth) clip goes from 0.0 (close) to 1.0 (far).
#[inline]
pub fn perspective_vk(vertical_fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Mat4 {
    let t = (vertical_fov / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;
    let nmf = z_near - z_far;

    Mat4::new(
        Vec4::new(sx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, -sy, 0.0, 0.0),
        Vec4::new(0.0, 0.0, z_far / nmf, z_near * z_far / nmf),
        Vec4::new(0.0, 0.0, -1.0, 0.0),
    )
}

/// Reversed-Z projection matrix meant to be used to convert a right-handed, y-up
/// coordinate space (the standard computer graphics coordinate space) to be used with OpenGL or DirectX,
/// which has a left-handed, y-up coordinate space where Z (depth) clip goes from 0.0 (close) to 1.0 (far).
///
/// Reversed-Z provides significantly better precision and therefore reduced z-fighting
/// for most depth situations, especially when a floating-point depth buffer is used. You'll want to use
/// a reversed depth comparison function and depth clear value when using this projection.
///
/// **Note that in order for this to work properly with OpenGL, you'll need to use the `gl_arb_clip_control` extension
/// and set the z clip from 0.0 to 1.0 rather than the default -1.0 to 1.0**
#[inline]
pub fn perspective_reversed_z_dx_gl(
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
        Vec4::new(0.0, 0.0, -z_far / nmf - 1.0, -z_near * z_far / nmf),
        Vec4::new(0.0, 0.0, -1.0, 0.0),
    )
}

/// Reversed-Z projection matrix meant to be used to convert a right-handed, y-up
/// coordinate space (the standard computer graphics coordinate space) to be used with Vulkan,
/// which has a right-handed, y-down coordinate space where Z (depth) clip goes from 0.0 (close) to 1.0 (far).
///
/// Reversed-Z provides significantly better precision and therefore reduced z-fighting
/// for most depth situations, especially when a floating-point depth buffer is used. You'll want to use
/// a reversed depth comparison function and depth clear value when using this projection.
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
        Vec4::new(0.0, 0.0, z_far / nmf, z_near * z_far / nmf),
        Vec4::new(0.0, 0.0, -1.0, 0.0),
    )
}

/// Reversed, infinite far-z projection matrix meant to be used to convert a right-handed, y-up
/// coordinate space (the standard computer graphics coordinate space) to be used with OpenGL or DirectX,
/// which has a left-handed, y-up coordinate space where Z (depth) clip goes from 0.0 (close) to 1.0 (far).
///
/// Reversed-Z provides significantly better precision and therefore reduced z-fighting
/// for most depth situations, especially when a floating-point depth buffer is used. You'll want to use
/// a reversed depth comparison function and depth clear value when using this projection.
///
/// Infinite far-z is useful for extremely large scenes where having a far clip plane would not be useful as it eliminates
/// several approximate numerical computations and so can improve z-fighting behavior further.
///
/// **Note that in order for this to work properly with OpenGL, you'll need to use the `gl_arb_clip_control` extension
/// and set the z clip from 0.0 to 1.0 rather than the default -1.0 to 1.0**
#[inline]
pub fn perspective_reversed_infinite_z_dx_gl(
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
        Vec4::new(0.0, 0.0, 0.0, z_near),
        Vec4::new(0.0, 0.0, -1.0, 0.0),
    )
}

/// Reversed-Z projection matrix meant to be used to convert a right-handed, y-up
/// coordinate space (the standard computer graphics coordinate space) to be used with Vulkan,
/// which has a right-handed, y-down coordinate space where Z (depth) clip goes from 0.0 (close) to 1.0 (far).
///
/// Reversed-Z provides significantly better precision and therefore reduced z-fighting
/// for most depth situations, especially when a floating-point depth buffer is used. You'll want to use
/// a reversed depth comparison function and depth clear value when using this projection.
///
/// Infinite far-z is useful for extremely large scenes where having a far clip plane would not be useful as it eliminates
/// several approximate numerical computations and so can improve z-fighting behavior further.
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
        Vec4::new(0.0, 0.0, 0.0, z_near),
        Vec4::new(0.0, 0.0, -1.0, 0.0),
    )
}
