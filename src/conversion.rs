use crate::*;
use core::convert::TryFrom;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FloatErrorKind {
    NaN,
    Infinite,
    PosOverflow,
    NegOverflow,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FloatConversionError {
    pub kind: FloatErrorKind,
}

impl FloatConversionError {
    pub const fn nan() -> Self {
        Self {
            kind: FloatErrorKind::NaN,
        }
    }

    pub const fn infinite() -> Self {
        Self {
            kind: FloatErrorKind::Infinite,
        }
    }

    pub const fn pos_overflow() -> Self {
        Self {
            kind: FloatErrorKind::PosOverflow,
        }
    }

    pub const fn neg_overflow() -> Self {
        Self {
            kind: FloatErrorKind::NegOverflow,
        }
    }
}

// Not public to not leak outside.
trait TryFromExt<Source>: Sized {
    type Error;

    fn try_from(source: Source) -> Result<Self, Self::Error>;
}

// Not public to not leak outside.
trait TryIntoExt<Target> {
    type Error;

    fn try_into(self) -> Result<Target, Self::Error>;
}

// Generic implementation
impl<Source, Target, E> TryIntoExt<Target> for Source
where
    Target: TryFromExt<Source, Error = E>,
{
    type Error = E;

    fn try_into(self) -> Result<Target, Self::Error> {
        Target::try_from(self)
    }
}

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
                    return Err(FloatConversionError::nan())
                }
                if source.is_infinite() {
                    return Err(FloatConversionError::infinite())
                }

                let min = Self::MIN as $source;
                let max = Self::MAX as $source;

                if source < min {
                    return Err(FloatConversionError::neg_overflow())
                }
                if source > max {
                    return Err(FloatConversionError::pos_overflow())
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

        assert!(ivec2.is_ok());
        assert_eq!(ivec2.ok().unwrap(), IVec2::new(1, 2));
    }

    #[test]
    #[cfg(feature = "int")]
    fn vec2_to_ivec2_floored() {
        let vec2 = Vec2::new(1.99, 2.99);
        let ivec2 = IVec2::try_from(vec2);

        assert!(ivec2.is_ok());
        assert_eq!(ivec2.ok().unwrap(), IVec2::new(1, 2));
    }

    #[test]
    #[cfg(feature = "int")]
    fn vec2_to_ivec2_nan() {
        let vec2 = Vec2::new(f32::NAN, 0.0);
        let ivec2 = IVec2::try_from(vec2);

        assert!(ivec2.is_err());
        assert_eq!(ivec2.err().unwrap(), FloatConversionError::nan());
    }

    #[test]
    #[cfg(feature = "int")]
    fn vec2_to_ivec2_infinity() {
        let vec2 = Vec2::new(f32::INFINITY, 0.0);
        let ivec2 = IVec2::try_from(vec2);

        assert!(ivec2.is_err());
        assert_eq!(ivec2.err().unwrap(), FloatConversionError::infinite());
    }

    #[test]
    #[cfg(feature = "int")]
    fn vec2_to_ivec2_neg_infinity() {
        let vec2 = Vec2::new(f32::NEG_INFINITY, 0.0);
        let ivec2 = IVec2::try_from(vec2);

        assert!(ivec2.is_err());
        assert_eq!(ivec2.err().unwrap(), FloatConversionError::infinite());
    }

    #[test]
    #[cfg(feature = "int")]
    fn vec2_to_ivec2_pos_overflow() {
        let vec2 = Vec2::new(f32::MAX, 0.0);
        let ivec2 = IVec2::try_from(vec2);

        assert!(ivec2.is_err());
        assert_eq!(ivec2.err().unwrap(), FloatConversionError::pos_overflow());
    }

    #[test]
    #[cfg(feature = "int")]
    fn vec2_to_ivec2_neg_overflow() {
        let vec2 = Vec2::new(f32::MIN, 0.0);
        let ivec2 = IVec2::try_from(vec2);

        assert!(ivec2.is_err());
        assert_eq!(ivec2.err().unwrap(), FloatConversionError::neg_overflow());
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

        assert!(uvec2.is_err());
        assert_eq!(uvec2.err().unwrap(), FloatConversionError::neg_overflow());
    }
}
