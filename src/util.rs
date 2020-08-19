use wide::f32x4;

pub trait EqualsEps {
    fn eq_eps(self, other: Self) -> bool;
}

impl EqualsEps for f32x4 {
    fn eq_eps(self, other: Self) -> bool {
        let r = (self - other).abs();
        let eps = f32x4::from(0.01);

        let mask = r.cmp_ge(eps).move_mask();

        mask == 0b0000
    }
}

impl EqualsEps for f32 {
    fn eq_eps(self, other: Self) -> bool {
        (self - other).abs() <= 0.01
    }
}

#[macro_export]
macro_rules! derive_default_identity {
    ($t:ident) => {
        impl Default for $t {
            #[inline]
            fn default() -> Self {
                Self::identity()
            }
        }
    };
}
