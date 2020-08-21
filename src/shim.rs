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

macro_rules! impl_blend_shim {
    ($(($mask_t:ident, $vec_t:ident)),+) => {
        $(impl BlendShim<$vec_t> for $mask_t {
            fn blend(self, tru: $vec_t, fals: $vec_t) -> $vec_t {
                self.select(tru, fals)
            }
        })+
    }
}

impl_blend_shim!(
    (m32x4, f32x4),
    (m32x8, f32x8),
    (m32x16, f32x16),
    (m64x2, f64x2),
    (m64x4, f64x4),
    (m64x8, f64x8)
);
