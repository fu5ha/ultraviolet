//// Projective geometry algebry concerns about flattening or projecting vectors
/// onto other vectors or planes.
pub trait Project {
    /// Projects the vector self onto `other`.
    /// Assumes `other` is normalized.
    fn proj(&self, other: Self) -> Self;

    /// Projects the vector onto the plane defined by `normal`.
    /// This is also called rejective projection
    /// Asssumes other is normalized.
    fn proj_plane(&self, normal: Self) -> Self;
}

macro_rules! impl_project {
    () => {};
    ($ty: ident) => {
        impl crate::Project for $ty {
            fn proj(&self, other: Self) -> Self {
                other * other.dot(*self)
            }

            fn proj_plane(&self, normal: Self) -> Self {
                *self - normal * normal.dot(*self)
            }
        }
    };

    ([$($rest: ident), *]) => {
        $(
            impl_project!($rest);
        )*
};
}

use crate::*;

impl_project!([Vec2, Vec3, Vec4, Vec2x8, Vec3x8, Vec4x8, Vec2x4, Vec3x4, Vec4x4]);
#[cfg(feature = "int")]
impl_project!([IVec2, IVec3, IVec4, UVec2, UVec3, UVec4]);

#[cfg(feature = "f64")]
impl_project!([DVec2, DVec3, DVec4, DVec2x2, DVec3x2, DVec4x2, DVec2x4, DVec3x4, DVec4x4]);

#[cfg(test)]
mod tests {
    use crate::{Project, Vec3};

    #[test]
    fn project() {
        let a = Vec3::new(1.0, 4.0, 3.0);
        let b = Vec3::unit_y();

        assert_eq!(a.proj(b), Vec3::new(0.0, 4.0, 0.0));
        assert_eq!(a.proj_plane(b), Vec3::new(1.0, 0.0, 3.0));
    }

    #[test]
    #[cfg(feature = "int")]
    fn project_int() {
        use crate::IVec4;
        let a = IVec4::new(1, 4, 3, 6);
        let b = IVec4::unit_y();

        assert_eq!(a.proj(b), IVec4::new(0, 4, 0, 0));
        assert_eq!(a.proj_plane(b), IVec4::new(1, 0, 3, 6));
    }
}
