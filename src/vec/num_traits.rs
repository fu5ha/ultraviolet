use crate::*;

macro_rules! impl_num_traits_vecs {
    ($($n:ident),+) => {
        $(
        impl num_traits::Zero for $n {
            #[inline]
            fn zero() -> Self {
                $n::zero()
            }

            #[inline]
            fn is_zero(&self) -> bool {
                &$n::zero() == self
            }
        }

        impl num_traits::One for $n {
            #[inline]
            fn one() -> Self {
                $n::one()
            }
        }
        )+
    };
}

impl_num_traits_vecs!(Vec2, Vec2x4, Vec2x8, Vec3, Vec3x4, Vec3x8, Vec4, Vec4x4, Vec4x8);

#[cfg(feature = "f64")]
impl_num_traits_vecs!(DVec2, DVec2x2, DVec2x4, DVec3, DVec3x2, DVec3x4, DVec4, DVec4x2, DVec4x4);
