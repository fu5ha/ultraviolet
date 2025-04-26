use crate::*;

pub(crate) trait Splat<T> {
    fn splat(val: T) -> Self;
}

impl Splat<f32> for f32 {
    #[inline(always)]
    fn splat(val: f32) -> Self {
        val
    }
}

impl Splat<f64> for f64 {
    #[inline(always)]
    fn splat(val: f64) -> Self {
        val
    }
}

pub trait EqualsEps {
    fn eq_eps(self, other: Self) -> bool;
}

macro_rules! impl_eq_eps_wide {
    ($($t:ident),+) => {
        $(impl EqualsEps for $t {
            fn eq_eps(self, other: Self) -> bool {
                let r = (self - other).abs();
                let eps = $t::splat(0.01);

                r.cmp_ge(eps).none()
            }
        })+
    };
}

impl_eq_eps_wide!(f32x4, f32x8, f64x2, f64x4);

impl EqualsEps for f32 {
    fn eq_eps(self, other: Self) -> bool {
        let diff = (self - other).abs();
        if diff <= 0.01 {
            true
        } else {
            println!(
                "{} should equal {} with epsilon 0.01 but doesn't.",
                self, other
            );
            false
        }
    }
}

impl EqualsEps for f64 {
    fn eq_eps(self, other: Self) -> bool {
        let diff = (self - other).abs();
        if diff <= 0.01 {
            true
        } else {
            println!(
                "{} should equal {} with epsilon 0.01 but doesn't.",
                self, other
            );
            false
        }
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
