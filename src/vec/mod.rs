//! Vectors and points, i.e. directed line segments and locations.
mod vec2;
mod vec3;
mod vec4;

pub use vec2::*;
pub use vec3::*;
pub use vec4::*;

pub trait MulAdd<A = Self, B = Self> {
    /// The resulting type after applying the fused multiply-add.
    type Output;

    /// Performs the fused multiply-add operation.
    fn mul_add(self, a: A, b: B) -> Self::Output;
}
