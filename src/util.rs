use wide::f32x4;

pub trait EqualsEps {
    fn eq_eps(self, other: Self) -> bool;
}

impl EqualsEps for f32x4 {
    fn eq_eps(self, other: Self) -> bool {
        let r = (self - other).abs();
        for eps in r.as_ref().iter() {
            if *eps > 0.01 {
                return false;
            }
        }
        true
    }
}

impl EqualsEps for f32 {
    fn eq_eps(self, other: Self) -> bool {
        (self - other).abs() <= 0.01
    }
}

impl EqualsEps for f64 {
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
    }
}
