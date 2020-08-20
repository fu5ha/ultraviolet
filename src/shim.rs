use crate::*;

pub(crate) trait CmpShim<MT> {
    fn cmp_eq(self: Self, rhs: Self) -> MT;
    fn cmp_gt(self: Self, rhs: Self) -> MT;
    fn cmp_ge(self: Self, rhs: Self) -> MT;
    fn cmp_lt(self: Self, rhs: Self) -> MT;
    fn cmp_le(self: Self, rhs: Self) -> MT;
}

macro_rules! impl_cmp_shim {
    ($(($mask_t:ident, $vec_t:ident)),+) => {
        $(impl CmpShim<$mask_t> for $vec_t {
            fn cmp_eq(self, rhs: Self) -> $mask_t {
                self.eq(rhs)
            }
            fn cmp_gt(self, rhs: Self) -> $mask_t {
                self.gt(rhs)
            }
            fn cmp_ge(self, rhs: Self) -> $mask_t {
                self.ge(rhs)
            }
            fn cmp_lt(self, rhs: Self) -> $mask_t {
                self.lt(rhs)
            }
            fn cmp_le(self, rhs: Self) -> $mask_t {
                self.le(rhs)
            }
        })+
    };
}

impl_cmp_shim!(
    (m32x4, f32x4),
    (m32x8, f32x8),
    (m32x16, f32x16),
    (m64x2, f64x2),
    (m64x4, f64x4),
    (m64x8, f64x8)
);

pub(crate) trait BlendShim<T> {
    fn blend(self: Self, tru: T, fals: T) -> T;
}

impl BlendShim<f32x4> for m32x4 {
    fn blend(self, tru: f32x4, fals: f32x4) -> f32x4 {
        self.select(tru, fals)
    }
}

impl BlendShim<f32x8> for m32x8 {
    fn blend(self, tru: f32x8, fals: f32x8) -> f32x8 {
        self.select(tru, fals)
    }
}

impl BlendShim<f64x2> for m64x2 {
    fn blend(self, tru: f64x2, fals: f64x2) -> f64x2 {
        self.select(tru, fals)
    }
}

impl BlendShim<f64x4> for m64x4 {
    fn blend(self, tru: f64x4, fals: f64x4) -> f64x4 {
        self.select(tru, fals)
    }
}
