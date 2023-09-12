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

#[cfg(test)]
macro_rules! assert_eq_eps {
    ($left:expr, $right:expr, $eps:expr) => {{
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(left_val.eq_eps(*right_val, $eps)) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(
                        r#"assertion failed: `(left ~= right with epsilon {})`
 left: `{:?}`,
 right: `{:?}`"#,
                        $eps, &*left_val, &*right_val
                    )
                }
            }
        }
    }};
}

pub trait EqualsEps {
    fn eq_eps(self, other: Self, eps: f32) -> bool;
}

macro_rules! impl_eq_eps_wide {
    ($($t:ident),+) => {
        $(impl EqualsEps for $t {
            fn eq_eps(self, other: Self, eps: f32) -> bool {
                let r = (self - other).abs();
                let eps = $t::splat(eps as _);

                r.cmp_ge(eps).none()
            }
        })+
    };
}

impl_eq_eps_wide!(f32x4, f32x8, f64x2, f64x4);

impl EqualsEps for f32 {
    fn eq_eps(self, other: Self, eps: f32) -> bool {
        (self - other).abs() <= eps
    }
}

impl EqualsEps for f64 {
    fn eq_eps(self, other: Self, eps: f32) -> bool {
        (self - other).abs() <= eps as f64
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

/// A simple trait extension to simulate `TryFrom` for types that are not from this crate.
pub trait TryFromExt<Source>: Sized {
    type Error;

    fn try_from(source: Source) -> Result<Self, Self::Error>;
}

/// A simple trait extension to simulate `TryInto` for types that are not from this crate.
pub trait TryIntoExt<Target> {
    type Error;

    fn try_into(self) -> Result<Target, Self::Error>;
}

impl<Source, Target, E> TryIntoExt<Target> for Source
where
    Target: TryFromExt<Source, Error = E>,
{
    type Error = E;

    fn try_into(self) -> Result<Target, Self::Error> {
        Target::try_from(self)
    }
}
