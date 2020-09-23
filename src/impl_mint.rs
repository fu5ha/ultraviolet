use crate::*;

macro_rules! from_vec2s {
    ($($minttype:ty => $uvtype:ty),+) => {
        $(impl From<$minttype> for $uvtype {
            #[inline]
            fn from(v: $minttype) -> Self {
                Self::new(v.x, v.y)
            }
        }

        impl From<$uvtype> for $minttype {
            #[inline]
            fn from(v: $uvtype) -> Self {
                Self {
                    x: v.x,
                    y: v.y,
                }
            }
        })+
    }
}

macro_rules! from_vec3s {
    ($($minttype:ty => $uvtype:ty),+) => {
        $(impl From<$minttype> for $uvtype {
            #[inline]
            fn from(v: $minttype) -> Self {
                Self::new(v.x, v.y, v.z)
            }
        }

        impl From<$uvtype> for $minttype {
            #[inline]
            fn from(v: $uvtype) -> Self {
                Self {
                    x: v.x,
                    y: v.y,
                    z: v.z,
                }
            }
        })+
    }
}

macro_rules! from_vec4s {
    ($($minttype:ty => $uvtype:ty),+) => {
        $(impl From<$minttype> for $uvtype {
            #[inline]
            fn from(v: $minttype) -> Self {
                Self::new(v.x, v.y, v.z, v.w)
            }
        }

        impl From<$uvtype> for $minttype {
            #[inline]
            fn from(v: $uvtype) -> Self {
                Self {
                    x: v.x,
                    y: v.y,
                    z: v.z,
                    w: v.w,
                }
            }
        })+
    }
}

from_vec2s!(
    mint::Vector2<f32> => Vec2,
    mint::Point2<f32> => Vec2
);
#[cfg(feature = "f64")]
from_vec2s!(
    mint::Vector2<f64> => DVec2,
    mint::Point2<f64> => DVec2
);

from_vec3s!(
    mint::Vector3<f32> => Vec3,
    mint::Point3<f32> => Vec3
);
#[cfg(feature = "f64")]
from_vec3s!(
    mint::Vector3<f64> => DVec3,
    mint::Point3<f64> => DVec3
);

from_vec4s!(mint::Vector4<f32> => Vec4);
#[cfg(feature = "f64")]
from_vec4s!(mint::Vector4<f64> => DVec4);

macro_rules! from_mat2s {
    ($($minttype:ty => $uvtype:ty),+) => {
        $(impl From<$minttype> for $uvtype {
            #[inline]
            fn from(v: $minttype) -> Self {
                Self::new(v.x.into(), v.y.into())
            }
        }

        impl From<$uvtype> for $minttype {
            #[inline]
            fn from(v: $uvtype) -> Self {
                Self {
                    x: v.cols[0].into(),
                    y: v.cols[1].into(),
                }
            }
        })+
    }
}

macro_rules! from_mat3s {
    ($($minttype:ty => $uvtype:ty),+) => {
        $(impl From<$minttype> for $uvtype {
            #[inline]
            fn from(v: $minttype) -> Self {
                Self::new(v.x.into(), v.y.into(), v.z.into())
            }
        }

        impl From<$uvtype> for $minttype {
            #[inline]
            fn from(v: $uvtype) -> Self {
                Self {
                    x: v.cols[0].into(),
                    y: v.cols[1].into(),
                    z: v.cols[2].into(),
                }
            }
        })+
    }
}

macro_rules! from_mat4s {
    ($($minttype:ty => $uvtype:ty),+) => {
        $(impl From<$minttype> for $uvtype {
            #[inline]
            fn from(v: $minttype) -> Self {
                Self::new(v.x.into(), v.y.into(), v.z.into(), v.w.into())
            }
        }

        impl From<$uvtype> for $minttype {
            #[inline]
            fn from(v: $uvtype) -> Self {
                Self {
                    x: v.cols[0].into(),
                    y: v.cols[1].into(),
                    z: v.cols[2].into(),
                    w: v.cols[3].into(),
                }
            }
        })+
    }
}

from_mat2s!(mint::ColumnMatrix2<f32> => Mat2);
#[cfg(feature = "f64")]
from_mat2s!(mint::ColumnMatrix2<f64> => DMat2);

from_mat3s!(mint::ColumnMatrix3<f32> => Mat3);
#[cfg(feature = "f64")]
from_mat3s!(mint::ColumnMatrix3<f64> => DMat3);

from_mat4s!(mint::ColumnMatrix4<f32> => Mat4);
#[cfg(feature = "f64")]
from_mat4s!(mint::ColumnMatrix4<f64> => DMat4);
