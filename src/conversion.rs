//! Contains implementations to convert between `UVec`/`IVec` and `Vec`/`DVec`.
//!
//! To realize such conversions we make use of crate-private traits `TryFromExt` and `TryIntoExt` to
//! simulate the behaviour of the official [From] and [Into].

use crate::util::{TryFromExt, TryIntoExt};
use crate::*;
use core::convert::TryFrom;
use std::error::Error;
use std::fmt;

/// The error type that may happen when converting a `f32` or `f64` to any other numerical
/// representation.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FloatConversionError {
    NaN,
    Infinite,
    PosOverflow,
    NegOverflow,
}

impl fmt::Display for FloatConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FloatConversionError::NaN => f.write_str("NaN"),
            FloatConversionError::Infinite => f.write_str("Infinite"),
            FloatConversionError::PosOverflow => f.write_str("PosOverflow"),
            FloatConversionError::NegOverflow => f.write_str("NegOverflow"),
        }
    }
}

impl Error for FloatConversionError {}

macro_rules! impl_try_from_float {
    ($source:ty => $($target:ident),*) => {$(
        impl TryFromExt<$source> for $target {
            type Error = FloatConversionError;

            /// Tries to convert the source to Self in a lossy way, flooring any float value.
            ///
            /// # Errors
            /// * `NaN` - If the float value is `NaN`.
            /// * `Infinite` - If the float value is infinity or negative infinity.
            /// * `PosOverflow` - If the float value would be greater than the target max value.
            /// * `NegOverflow` - If the float value would be less than the target min value.
            #[inline]
            fn try_from(source: $source) -> Result<Self, Self::Error> {
                if source.is_nan() {
                    return Err(FloatConversionError::NaN)
                }
                if source.is_infinite() {
                    return Err(FloatConversionError::Infinite)
                }

                let min = Self::MIN as $source;
                if source < min {
                    return Err(FloatConversionError::NegOverflow)
                }

                let max = Self::MAX as $source;
                if source > max {
                    return Err(FloatConversionError::PosOverflow)
                }

                Ok(source as Self)
            }
        }
    )*}
}

impl_try_from_float!(f32 => i32, u32);
impl_try_from_float!(f64 => i32, u32);

macro_rules! impl_try_from_float_vec {
    ($(($name:ident => $target:ident, [$($var:ident),*])),+) => {
        $(
        impl TryFrom<$name> for $target {
            type Error = FloatConversionError;

            /// Tries to convert the source to Self in a lossy way, flooring any float value.
            ///
            /// # Errors
            /// * `NaN` - If a float value is `NaN`.
            /// * `NotFinite` - If a float value is infinity or negative infinity.
            /// * `PosOverflow` - If a float value would be greater than the the self.component max value.
            /// * `NegOverflow` - If a float value would be less than the self.component min value.
            #[inline]
            fn try_from(v: $name) -> Result<Self, Self::Error> {
                Ok(Self::new($(v.$var.try_into()?,)* ))
            }
        }
        )+
    }
}

macro_rules! impl_from_int_vec {
    ($(($name:ident => $target:ident, $target_type:ident, [$($var:ident),*])),+) => {
        $(
        impl From<$name> for $target {
            #[inline]
            fn from(v: $name) -> Self {
                Self::new($(v.$var as $target_type,)*)
            }
        }
        )+
    };
}

impl_try_from_float_vec!(
    (Vec2 => IVec2, [x, y]),
    (Vec3 => IVec3, [x, y, z]),
    (Vec4 => IVec4, [x, y, z, w]),

    (Vec2 => UVec2, [x, y]),
    (Vec3 => UVec3, [x, y, z]),
    (Vec4 => UVec4, [x, y, z, w])
);

#[cfg(feature = "f64")]
impl_try_from_float_vec!(
    (DVec2 => IVec2, [x, y]),
    (DVec3 => IVec3, [x, y, z]),
    (DVec4 => IVec4, [x, y, z, w]),

    (DVec2 => UVec2, [x, y]),
    (DVec3 => UVec3, [x, y, z]),
    (DVec4 => UVec4, [x, y, z, w])
);

impl_from_int_vec!(
    (IVec2 => Vec2, f32, [x, y]),
    (IVec3 => Vec3, f32, [x, y, z]),
    (IVec4 => Vec4, f32, [x, y, z, w]),

    (UVec2 => Vec2, f32, [x, y]),
    (UVec3 => Vec3, f32, [x, y, z]),
    (UVec4 => Vec4, f32, [x, y, z, w])
);

#[cfg(feature = "f64")]
impl_from_int_vec!(
    (IVec2 => DVec2, f64, [x, y]),
    (IVec3 => DVec3, f64, [x, y, z]),
    (IVec4 => DVec4, f64, [x, y, z, w]),

    (UVec2 => DVec2, f64, [x, y]),
    (UVec3 => DVec3, f64, [x, y, z]),
    (UVec4 => DVec4, f64, [x, y, z, w])
);

// tests only for Vec2
#[cfg(test)]
mod tests {
    use crate::*;
    use core::convert::TryFrom;

    #[test]
    #[cfg(feature = "int")]
    fn vec2_to_ivec2_exact() {
        let vec2 = Vec2::new(1.0, 2.0);
        let ivec2 = IVec2::try_from(vec2);

        assert_eq!(ivec2.ok().unwrap(), IVec2::new(1, 2));
    }

    #[test]
    #[cfg(feature = "int")]
    fn vec2_to_ivec2_floored() {
        let vec2 = Vec2::new(1.99, 2.99);
        let ivec2 = IVec2::try_from(vec2);

        assert_eq!(ivec2.ok().unwrap(), IVec2::new(1, 2));
    }

    #[test]
    #[cfg(feature = "int")]
    fn vec2_to_ivec2_nan() {
        let vec2 = Vec2::new(f32::NAN, 0.0);
        let ivec2 = IVec2::try_from(vec2);

        assert_eq!(ivec2.err().unwrap(), FloatConversionError::NaN);
    }

    #[test]
    #[cfg(feature = "int")]
    fn vec2_to_ivec2_infinity() {
        let vec2 = Vec2::new(f32::INFINITY, 0.0);
        let ivec2 = IVec2::try_from(vec2);

        assert_eq!(ivec2.err().unwrap(), FloatConversionError::Infinite);
    }

    #[test]
    #[cfg(feature = "int")]
    fn vec2_to_ivec2_neg_infinity() {
        let vec2 = Vec2::new(f32::NEG_INFINITY, 0.0);
        let ivec2 = IVec2::try_from(vec2);

        assert_eq!(ivec2.err().unwrap(), FloatConversionError::Infinite);
    }

    #[test]
    #[cfg(feature = "int")]
    fn vec2_to_ivec2_pos_overflow() {
        let vec2 = Vec2::new(f32::MAX, 0.0);
        let ivec2 = IVec2::try_from(vec2);

        assert_eq!(ivec2.err().unwrap(), FloatConversionError::PosOverflow);
    }

    #[test]
    #[cfg(feature = "int")]
    fn vec2_to_ivec2_neg_overflow() {
        let vec2 = Vec2::new(f32::MIN, 0.0);
        let ivec2 = IVec2::try_from(vec2);

        assert_eq!(ivec2.err().unwrap(), FloatConversionError::NegOverflow);
    }

    #[test]
    #[cfg(feature = "int")]
    fn ivec2_to_vec2() {
        let ivec2 = IVec2::new(1, 2);
        let vec2 = Vec2::from(ivec2);

        assert_eq!(vec2, Vec2::new(1.0, 2.0));
    }

    #[test]
    #[cfg(feature = "int")]
    fn vec2_to_uvec2_neg_overflow() {
        let vec2 = Vec2::new(-1.0, 0.0);
        let uvec2 = UVec2::try_from(vec2);

        assert_eq!(uvec2.err().unwrap(), FloatConversionError::NegOverflow);
    }
}
