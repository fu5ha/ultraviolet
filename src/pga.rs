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

use crate::{Vec2, Vec3, Vec4};

impl_project!([Vec2, Vec3, Vec4]);
#[cfg(feature = "int")]
mod int {
    use crate::{IVec2, IVec3, IVec4, UVec2, UVec3, UVec4};
    impl_project!([IVec2, IVec3, IVec4, UVec2, UVec3, UVec4]);
}

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
