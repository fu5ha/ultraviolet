use crate::*;

use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};

macro_rules! impl_serde_vec2 {
    ($t:ident, $n:expr) => {
        impl Serialize for $t {
            fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
            where
                T: Serializer,
            {
                let mut state = serializer.serialize_struct($n, 2)?;
                state.serialize_field("x", &self.x)?;
                state.serialize_field("y", &self.y)?;
                state.end()
            }
        }

        impl<'de> Deserialize<'de> for $t {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                enum Field {
                    X,
                    Y,
                }

                impl<'de> Deserialize<'de> for Field {
                    fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        struct FieldVisitor;

                        impl<'de> Visitor<'de> for FieldVisitor {
                            type Value = Field;

                            fn expecting(
                                &self,
                                formatter: &mut std::fmt::Formatter,
                            ) -> std::fmt::Result {
                                formatter.write_str("`x` or `y`")
                            }

                            fn visit_str<E>(self, value: &str) -> Result<Field, E>
                            where
                                E: serde::de::Error,
                            {
                                match value {
                                    "x" => Ok(Field::X),
                                    "y" => Ok(Field::Y),
                                    _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                }
                            }
                        }

                        deserializer.deserialize_identifier(FieldVisitor)
                    }
                }

                struct TVisitor;

                impl<'de> Visitor<'de> for TVisitor {
                    type Value = $t;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(&["struct ", $n].concat())
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<$t, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let x = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                        let y = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                        Ok($t::new(x, y))
                    }

                    fn visit_map<V>(self, mut map: V) -> Result<$t, V::Error>
                    where
                        V: MapAccess<'de>,
                    {
                        let mut x = None;
                        let mut y = None;
                        while let Some(key) = map.next_key()? {
                            match key {
                                Field::X => {
                                    if x.is_some() {
                                        return Err(serde::de::Error::duplicate_field("x"));
                                    }
                                    x = Some(map.next_value()?);
                                }
                                Field::Y => {
                                    if y.is_some() {
                                        return Err(serde::de::Error::duplicate_field("y"));
                                    }
                                    y = Some(map.next_value()?);
                                }
                            }
                        }
                        let x = x.ok_or_else(|| serde::de::Error::missing_field("x"))?;
                        let y = y.ok_or_else(|| serde::de::Error::missing_field("y"))?;
                        Ok($t::new(x, y))
                    }
                }

                const FIELDS: &'static [&'static str] = &["x", "y"];

                deserializer.deserialize_struct($n, FIELDS, TVisitor)
            }
        }
    };
}
macro_rules! impl_serde_vec3 {
    ($t:ident, $n:expr) => {
        impl Serialize for $t {
            fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
            where
                T: Serializer,
            {
                let mut state = serializer.serialize_struct($n, 3)?;
                state.serialize_field("x", &self.x)?;
                state.serialize_field("y", &self.y)?;
                state.serialize_field("z", &self.z)?;
                state.end()
            }
        }

        impl<'de> Deserialize<'de> for $t {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                enum Field {
                    X,
                    Y,
                    Z,
                }

                impl<'de> Deserialize<'de> for Field {
                    fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        struct FieldVisitor;

                        impl<'de> Visitor<'de> for FieldVisitor {
                            type Value = Field;

                            fn expecting(
                                &self,
                                formatter: &mut std::fmt::Formatter,
                            ) -> std::fmt::Result {
                                formatter.write_str("`x` or `y` or `z`")
                            }

                            fn visit_str<E>(self, value: &str) -> Result<Field, E>
                            where
                                E: serde::de::Error,
                            {
                                match value {
                                    "x" => Ok(Field::X),
                                    "y" => Ok(Field::Y),
                                    "z" => Ok(Field::Z),
                                    _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                }
                            }
                        }

                        deserializer.deserialize_identifier(FieldVisitor)
                    }
                }

                struct TVisitor;

                impl<'de> Visitor<'de> for TVisitor {
                    type Value = $t;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(&["struct ", $n].concat())
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<$t, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let x = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                        let y = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                        let z = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                        Ok($t::new(x, y, z))
                    }

                    fn visit_map<V>(self, mut map: V) -> Result<$t, V::Error>
                    where
                        V: MapAccess<'de>,
                    {
                        let mut x = None;
                        let mut y = None;
                        let mut z = None;
                        while let Some(key) = map.next_key()? {
                            match key {
                                Field::X => {
                                    if x.is_some() {
                                        return Err(serde::de::Error::duplicate_field("x"));
                                    }
                                    x = Some(map.next_value()?);
                                }
                                Field::Y => {
                                    if y.is_some() {
                                        return Err(serde::de::Error::duplicate_field("y"));
                                    }
                                    y = Some(map.next_value()?);
                                }
                                Field::Z => {
                                    if z.is_some() {
                                        return Err(serde::de::Error::duplicate_field("z"));
                                    }
                                    z = Some(map.next_value()?);
                                }
                            }
                        }
                        let x = x.ok_or_else(|| serde::de::Error::missing_field("x"))?;
                        let y = y.ok_or_else(|| serde::de::Error::missing_field("y"))?;
                        let z = z.ok_or_else(|| serde::de::Error::missing_field("z"))?;
                        Ok($t::new(x, y, z))
                    }
                }

                const FIELDS: &'static [&'static str] = &["x", "y", "z"];

                deserializer.deserialize_struct($n, FIELDS, TVisitor)
            }
        }
    };
}
macro_rules! impl_serde_vec4 {
    ($t:ident, $n:expr) => {
        impl Serialize for $t {
            fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
            where
                T: Serializer,
            {
                let mut state = serializer.serialize_struct($n, 4)?;
                state.serialize_field("x", &self.x)?;
                state.serialize_field("y", &self.y)?;
                state.serialize_field("z", &self.z)?;
                state.serialize_field("w", &self.w)?;
                state.end()
            }
        }

        impl<'de> Deserialize<'de> for $t {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                enum Field {
                    X,
                    Y,
                    Z,
                    W,
                }

                impl<'de> Deserialize<'de> for Field {
                    fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        struct FieldVisitor;

                        impl<'de> Visitor<'de> for FieldVisitor {
                            type Value = Field;

                            fn expecting(
                                &self,
                                formatter: &mut std::fmt::Formatter,
                            ) -> std::fmt::Result {
                                formatter.write_str("`x` or `y` or `z` or `w`")
                            }

                            fn visit_str<E>(self, value: &str) -> Result<Field, E>
                            where
                                E: serde::de::Error,
                            {
                                match value {
                                    "x" => Ok(Field::X),
                                    "y" => Ok(Field::Y),
                                    "z" => Ok(Field::Z),
                                    "w" => Ok(Field::W),
                                    _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                }
                            }
                        }

                        deserializer.deserialize_identifier(FieldVisitor)
                    }
                }

                struct TVisitor;

                impl<'de> Visitor<'de> for TVisitor {
                    type Value = $t;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(&["struct ", $n].concat())
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<$t, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let x = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                        let y = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                        let z = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                        let w = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(3, &self))?;
                        Ok($t::new(x, y, z, w))
                    }

                    fn visit_map<V>(self, mut map: V) -> Result<$t, V::Error>
                    where
                        V: MapAccess<'de>,
                    {
                        let mut x = None;
                        let mut y = None;
                        let mut z = None;
                        let mut w = None;
                        while let Some(key) = map.next_key()? {
                            match key {
                                Field::X => {
                                    if x.is_some() {
                                        return Err(serde::de::Error::duplicate_field("x"));
                                    }
                                    x = Some(map.next_value()?);
                                }
                                Field::Y => {
                                    if y.is_some() {
                                        return Err(serde::de::Error::duplicate_field("y"));
                                    }
                                    y = Some(map.next_value()?);
                                }
                                Field::Z => {
                                    if z.is_some() {
                                        return Err(serde::de::Error::duplicate_field("z"));
                                    }
                                    z = Some(map.next_value()?);
                                }
                                Field::W => {
                                    if w.is_some() {
                                        return Err(serde::de::Error::duplicate_field("w"));
                                    }
                                    w = Some(map.next_value()?);
                                }
                            }
                        }
                        let x = x.ok_or_else(|| serde::de::Error::missing_field("x"))?;
                        let y = y.ok_or_else(|| serde::de::Error::missing_field("y"))?;
                        let z = z.ok_or_else(|| serde::de::Error::missing_field("z"))?;
                        let w = w.ok_or_else(|| serde::de::Error::missing_field("w"))?;
                        Ok($t::new(x, y, z, w))
                    }
                }

                const FIELDS: &'static [&'static str] = &["x", "y", "z", "w"];

                deserializer.deserialize_struct($n, FIELDS, TVisitor)
            }
        }
    };
}

impl_serde_vec2!(Vec2, "Vec2");
#[cfg(feature = "int")]
impl_serde_vec2!(UVec2, "UVec2");
#[cfg(feature = "int")]
impl_serde_vec2!(IVec2, "IVec2");
#[cfg(feature = "f64")]
impl_serde_vec2!(DVec2, "DVec2");

impl_serde_vec3!(Vec3, "Vec3");
#[cfg(feature = "int")]
impl_serde_vec3!(UVec3, "UVec3");
#[cfg(feature = "int")]
impl_serde_vec3!(IVec3, "IVec3");
#[cfg(feature = "f64")]
impl_serde_vec3!(DVec3, "DVec3");

impl_serde_vec4!(Vec4, "Vec4");
#[cfg(feature = "int")]
impl_serde_vec4!(UVec4, "UVec4");
#[cfg(feature = "int")]
impl_serde_vec4!(IVec4, "IVec4");
#[cfg(feature = "f64")]
impl_serde_vec4!(DVec4, "DVec4");

#[cfg(test)]
mod vec_serde_tests {
    use crate::vec::{Vec2, Vec3, Vec4};
    use serde_test::{assert_tokens, Token};

    #[test]
    fn vec2() {
        let vec2 = Vec2::new(1.0, 2.0);

        assert_tokens(
            &vec2,
            &[
                Token::Struct {
                    name: "Vec2",
                    len: 2,
                },
                Token::Str("x"),
                Token::F32(1.0),
                Token::Str("y"),
                Token::F32(2.0),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn vec3() {
        let vec3 = Vec3::new(1.0, 2.0, 3.0);

        assert_tokens(
            &vec3,
            &[
                Token::Struct {
                    name: "Vec3",
                    len: 3,
                },
                Token::Str("x"),
                Token::F32(1.0),
                Token::Str("y"),
                Token::F32(2.0),
                Token::Str("z"),
                Token::F32(3.0),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn vec4() {
        let vec4 = Vec4::new(1.0, 2.0, 3.0, 4.0);

        assert_tokens(
            &vec4,
            &[
                Token::Struct {
                    name: "Vec4",
                    len: 4,
                },
                Token::Str("x"),
                Token::F32(1.0),
                Token::Str("y"),
                Token::F32(2.0),
                Token::Str("z"),
                Token::F32(3.0),
                Token::Str("w"),
                Token::F32(4.0),
                Token::StructEnd,
            ],
        );
    }
}

#[cfg(feature = "int")]
#[cfg(test)]
mod int_vec_serde_tests {
    use crate::{IVec2, IVec3, IVec4, UVec2, UVec3, UVec4};
    use serde_test::{assert_tokens, Token};

    #[test]
    fn uvec2() {
        let vec2 = UVec2::new(1, 2);

        assert_tokens(
            &vec2,
            &[
                Token::Struct {
                    name: "UVec2",
                    len: 2,
                },
                Token::Str("x"),
                Token::U32(1),
                Token::Str("y"),
                Token::U32(2),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn uvec3() {
        let vec3 = UVec3::new(1, 2, 3);

        assert_tokens(
            &vec3,
            &[
                Token::Struct {
                    name: "UVec3",
                    len: 3,
                },
                Token::Str("x"),
                Token::U32(1),
                Token::Str("y"),
                Token::U32(2),
                Token::Str("z"),
                Token::U32(3),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn uvec4() {
        let vec4 = UVec4::new(1, 2, 3, 4);

        assert_tokens(
            &vec4,
            &[
                Token::Struct {
                    name: "UVec4",
                    len: 4,
                },
                Token::Str("x"),
                Token::U32(1),
                Token::Str("y"),
                Token::U32(2),
                Token::Str("z"),
                Token::U32(3),
                Token::Str("w"),
                Token::U32(4),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn ivec2() {
        let vec2 = IVec2::new(1, 2);

        assert_tokens(
            &vec2,
            &[
                Token::Struct {
                    name: "IVec2",
                    len: 2,
                },
                Token::Str("x"),
                Token::I32(1),
                Token::Str("y"),
                Token::I32(2),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn ivec3() {
        let vec3 = IVec3::new(1, 2, 3);

        assert_tokens(
            &vec3,
            &[
                Token::Struct {
                    name: "IVec3",
                    len: 3,
                },
                Token::Str("x"),
                Token::I32(1),
                Token::Str("y"),
                Token::I32(2),
                Token::Str("z"),
                Token::I32(3),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn ivec4() {
        let vec4 = IVec4::new(1, 2, 3, 4);

        assert_tokens(
            &vec4,
            &[
                Token::Struct {
                    name: "IVec4",
                    len: 4,
                },
                Token::Str("x"),
                Token::I32(1),
                Token::Str("y"),
                Token::I32(2),
                Token::Str("z"),
                Token::I32(3),
                Token::Str("w"),
                Token::I32(4),
                Token::StructEnd,
            ],
        );
    }
}

macro_rules! impl_serde_mat2 {
    ($name:ident, $vec:ident, $content:ident, $expecting:expr) => {
        impl Serialize for $name {
            fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
            where
                T: Serializer,
            {
                use serde::ser::SerializeSeq;

                let mut seq = serializer.serialize_seq(Some(4))?;

                seq.serialize_element(&self.cols[0].x)?;
                seq.serialize_element(&self.cols[0].y)?;
                seq.serialize_element(&self.cols[1].x)?;
                seq.serialize_element(&self.cols[1].y)?;
                seq.end()
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Mat2Visitor;

                impl<'de> serde::de::Visitor<'de> for Mat2Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str($expecting)
                    }

                    #[inline]
                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: SeqAccess<'de>,
                    {
                        use serde::de::Error;

                        Ok(Self::Value {
                            cols: [
                                $vec::new(
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(0, &self)),
                                    },
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(1, &self)),
                                    },
                                ),
                                $vec::new(
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(2, &self)),
                                    },
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(3, &self)),
                                    },
                                ),
                            ],
                        })
                    }
                }

                deserializer.deserialize_tuple(4, Mat2Visitor)
            }

            //    @TODO I understand how to implement it in the context of arrays but not matrices
            //    fn deserialize_in_place<D>(
            //        deserializer: D,
            //        place: &mut Self,
            //    ) -> Result<(), <D as Deserializer<'de>>::Error>
            //    where
            //        D: Deserializer<'de>,
            //    {
            //        unimplemented!()
            //    }
        }
    };
}
macro_rules! impl_serde_mat3 {
    ($name:ident, $vec:ident, $content:ident, $expecting:expr) => {
        impl Serialize for $name {
            fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
            where
                T: Serializer,
            {
                use serde::ser::SerializeSeq;

                let mut seq = serializer.serialize_seq(Some(9))?;

                seq.serialize_element(&self.cols[0].x)?;
                seq.serialize_element(&self.cols[0].y)?;
                seq.serialize_element(&self.cols[0].z)?;
                seq.serialize_element(&self.cols[1].x)?;
                seq.serialize_element(&self.cols[1].y)?;
                seq.serialize_element(&self.cols[1].z)?;
                seq.serialize_element(&self.cols[2].x)?;
                seq.serialize_element(&self.cols[2].y)?;
                seq.serialize_element(&self.cols[2].z)?;
                seq.end()
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Mat3Visitor;

                impl<'de> serde::de::Visitor<'de> for Mat3Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str($expecting)
                    }

                    #[inline]
                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: SeqAccess<'de>,
                    {
                        use serde::de::Error;

                        Ok(Self::Value {
                            cols: [
                                $vec::new(
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(0, &self)),
                                    },
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(1, &self)),
                                    },
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(2, &self)),
                                    },
                                ),
                                $vec::new(
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(3, &self)),
                                    },
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(4, &self)),
                                    },
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(5, &self)),
                                    },
                                ),
                                $vec::new(
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(6, &self)),
                                    },
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(7, &self)),
                                    },
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(8, &self)),
                                    },
                                ),
                            ],
                        })
                    }
                }

                deserializer.deserialize_tuple(9, Mat3Visitor)
            }

            //    @TODO I understand how to implement it in the context of arrays but not matrices
            //    fn deserialize_in_place<D>(
            //        deserializer: D,
            //        place: &mut Self,
            //    ) -> Result<(), <D as Deserializer<'de>>::Error>
            //    where
            //        D: Deserializer<'de>,
            //    {
            //        unimplemented!()
            //    }
        }
    };
}
macro_rules! impl_serde_mat4 {
    ($name:ident, $vec:ident, $content:ident, $expecting:expr) => {
        impl Serialize for $name {
            fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
            where
                T: Serializer,
            {
                use serde::ser::SerializeSeq;

                let mut seq = serializer.serialize_seq(Some(16))?;

                seq.serialize_element(&self.cols[0].x)?;
                seq.serialize_element(&self.cols[0].y)?;
                seq.serialize_element(&self.cols[0].z)?;
                seq.serialize_element(&self.cols[0].w)?;
                seq.serialize_element(&self.cols[1].x)?;
                seq.serialize_element(&self.cols[1].y)?;
                seq.serialize_element(&self.cols[1].z)?;
                seq.serialize_element(&self.cols[1].w)?;
                seq.serialize_element(&self.cols[2].x)?;
                seq.serialize_element(&self.cols[2].y)?;
                seq.serialize_element(&self.cols[2].z)?;
                seq.serialize_element(&self.cols[2].w)?;
                seq.serialize_element(&self.cols[3].x)?;
                seq.serialize_element(&self.cols[3].y)?;
                seq.serialize_element(&self.cols[3].z)?;
                seq.serialize_element(&self.cols[3].w)?;
                seq.end()
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Mat4Visitor;

                impl<'de> serde::de::Visitor<'de> for Mat4Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str($expecting)
                    }

                    #[inline]
                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: SeqAccess<'de>,
                    {
                        use serde::de::Error;

                        Ok(Self::Value {
                            cols: [
                                $vec::new(
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(0, &self)),
                                    },
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(1, &self)),
                                    },
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(2, &self)),
                                    },
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(3, &self)),
                                    },
                                ),
                                $vec::new(
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(4, &self)),
                                    },
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(5, &self)),
                                    },
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(6, &self)),
                                    },
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(7, &self)),
                                    },
                                ),
                                $vec::new(
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(8, &self)),
                                    },
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(9, &self)),
                                    },
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(10, &self)),
                                    },
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(11, &self)),
                                    },
                                ),
                                $vec::new(
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(12, &self)),
                                    },
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(13, &self)),
                                    },
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(14, &self)),
                                    },
                                    match seq.next_element::<$content>()? {
                                        Some(val) => val,
                                        None => return Err(Error::invalid_length(15, &self)),
                                    },
                                ),
                            ],
                        })
                    }
                }

                deserializer.deserialize_tuple(16, Mat4Visitor)
            }

            //    @TODO I understand how to implement it in the context of arrays but not matrices
            //    fn deserialize_in_place<D>(
            //        deserializer: D,
            //        place: &mut Self,
            //    ) -> Result<(), <D as Deserializer<'de>>::Error>
            //    where
            //        D: Deserializer<'de>,
            //    {
            //        unimplemented!()
            //    }
        }
    };
}

// Allowing the $expected macro in case of extending to integer matrices
impl_serde_mat2!(Mat2, Vec2, f32, "array of 4 floats");
#[cfg(feature = "f64")]
impl_serde_mat2!(DMat2, DVec2, f64, "array of 4 floats");

impl_serde_mat3!(Mat3, Vec3, f32, "array of 9 floats");
#[cfg(feature = "f64")]
impl_serde_mat3!(DMat3, DVec3, f64, "array of 9 floats");

impl_serde_mat4!(Mat4, Vec4, f32, "array of 16 floats");
#[cfg(feature = "f64")]
impl_serde_mat4!(DMat4, DVec4, f64, "array of 16 floats");

#[cfg(test)]
mod mat_serde_tests {
    use crate::mat::{Mat2, Mat3, Mat4};
    use crate::vec::{Vec2, Vec3, Vec4};
    use serde_test::{assert_tokens, Token};

    #[test]
    fn mat2() {
        let mat2 = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0));

        assert_tokens(
            &mat2,
            &[
                Token::Seq { len: Some(4) },
                Token::F32(1.0),
                Token::F32(2.0),
                Token::F32(3.0),
                Token::F32(4.0),
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn mat3() {
        let mat3 = Mat3::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(4.0, 5.0, 6.0),
            Vec3::new(7.0, 8.0, 9.0),
        );

        assert_tokens(
            &mat3,
            &[
                Token::Seq { len: Some(9) },
                Token::F32(1.0),
                Token::F32(2.0),
                Token::F32(3.0),
                Token::F32(4.0),
                Token::F32(5.0),
                Token::F32(6.0),
                Token::F32(7.0),
                Token::F32(8.0),
                Token::F32(9.0),
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn mat4() {
        let mat4 = Mat4::new(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );

        assert_tokens(
            &mat4,
            &[
                Token::Seq { len: Some(16) },
                Token::F32(1.0),
                Token::F32(2.0),
                Token::F32(3.0),
                Token::F32(4.0),
                Token::F32(5.0),
                Token::F32(6.0),
                Token::F32(7.0),
                Token::F32(8.0),
                Token::F32(9.0),
                Token::F32(10.0),
                Token::F32(11.0),
                Token::F32(12.0),
                Token::F32(13.0),
                Token::F32(14.0),
                Token::F32(15.0),
                Token::F32(16.0),
                Token::SeqEnd,
            ],
        );
    }
}

impl Serialize for Bivec2 {
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        let mut state = serializer.serialize_struct("Bivec2", 1)?;
        state.serialize_field("xy", &self.xy)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Bivec2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Xy,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`xy`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "xy" => Ok(Field::Xy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct Bivec2Visitor;

        impl<'de> Visitor<'de> for Bivec2Visitor {
            type Value = Bivec2;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Bivec2")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Bivec2, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let xy = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                Ok(Bivec2::new(xy))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Bivec2, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut xy = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Xy => {
                            if xy.is_some() {
                                return Err(serde::de::Error::duplicate_field("xy"));
                            }
                            xy = Some(map.next_value()?);
                        }
                    }
                }
                let xy = xy.ok_or_else(|| serde::de::Error::missing_field("xy"))?;
                Ok(Bivec2::new(xy))
            }
        }

        const FIELDS: &[&str] = &["xy"];

        deserializer.deserialize_struct("Bivec2", FIELDS, Bivec2Visitor)
    }
}

impl Serialize for Bivec3 {
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        let mut state = serializer.serialize_struct("Bivec3", 3)?;
        state.serialize_field("xy", &self.xy)?;
        state.serialize_field("zx", &self.zx)?;
        state.serialize_field("yz", &self.yz)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Bivec3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Xy,
            Xz,
            Yz,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`xy` or `xz` or `yz`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "xy" => Ok(Field::Xy),
                            "xz" => Ok(Field::Xz),
                            "yz" => Ok(Field::Yz),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct Bivec3Visitor;

        impl<'de> Visitor<'de> for Bivec3Visitor {
            type Value = Bivec3;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Bivec3")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Bivec3, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let xy = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let xz = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let yz = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                Ok(Bivec3::new(xy, xz, yz))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Bivec3, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut xy = None;
                let mut xz = None;
                let mut yz = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Xy => {
                            if xy.is_some() {
                                return Err(serde::de::Error::duplicate_field("xy"));
                            }
                            xy = Some(map.next_value()?);
                        }
                        Field::Xz => {
                            if xz.is_some() {
                                return Err(serde::de::Error::duplicate_field("xz"));
                            }
                            xz = Some(map.next_value()?);
                        }
                        Field::Yz => {
                            if yz.is_some() {
                                return Err(serde::de::Error::duplicate_field("yz"));
                            }
                            yz = Some(map.next_value()?);
                        }
                    }
                }
                let xy = xy.ok_or_else(|| serde::de::Error::missing_field("xy"))?;
                let xz = xz.ok_or_else(|| serde::de::Error::missing_field("xz"))?;
                let yz = yz.ok_or_else(|| serde::de::Error::missing_field("yz"))?;
                Ok(Bivec3::new(xy, xz, yz))
            }
        }

        const FIELDS: &[&str] = &["xy", "xz", "yz"];

        deserializer.deserialize_struct("Bivec3", FIELDS, Bivec3Visitor)
    }
}

#[cfg(test)]
mod bivec_serde_tests {
    use crate::bivec::{Bivec2, Bivec3};
    use serde_test::{assert_tokens, Token};

    #[test]
    fn bivec2() {
        let bivec2 = Bivec2::new(0.78);

        assert_tokens(
            &bivec2,
            &[
                Token::Struct {
                    name: "Bivec2",
                    len: 1,
                },
                Token::Str("xy"),
                Token::F32(0.78),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn bivec3() {
        let bivec3 = Bivec3::new(0.78, 0.36, 0.63);

        assert_tokens(
            &bivec3,
            &[
                Token::Struct {
                    name: "Bivec3",
                    len: 3,
                },
                Token::Str("xy"),
                Token::F32(0.78),
                Token::Str("xz"),
                Token::F32(0.36),
                Token::Str("yz"),
                Token::F32(0.63),
                Token::StructEnd,
            ],
        );
    }
}

impl Serialize for Rotor2 {
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        let mut state = serializer.serialize_struct("Rotor2", 2)?;
        state.serialize_field("s", &self.s)?;
        state.serialize_field("bv", &self.bv)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Rotor2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            S,
            Bv,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`s` or `bv`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "s" => Ok(Field::S),
                            "bv" => Ok(Field::Bv),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct Rotor2Visitor;

        impl<'de> Visitor<'de> for Rotor2Visitor {
            type Value = Rotor2;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Rotor2")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Rotor2, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let s = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let bv = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                Ok(Rotor2::new(s, bv))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Rotor2, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut s = None;
                let mut bv = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::S => {
                            if s.is_some() {
                                return Err(serde::de::Error::duplicate_field("s"));
                            }
                            s = Some(map.next_value()?);
                        }
                        Field::Bv => {
                            if bv.is_some() {
                                return Err(serde::de::Error::duplicate_field("bv"));
                            }
                            bv = Some(map.next_value()?);
                        }
                    }
                }
                let s = s.ok_or_else(|| serde::de::Error::missing_field("s"))?;
                let bv = bv.ok_or_else(|| serde::de::Error::missing_field("bv"))?;
                Ok(Rotor2::new(s, bv))
            }
        }

        const FIELDS: &[&str] = &["s", "bv"];

        deserializer.deserialize_struct("Rotor2", FIELDS, Rotor2Visitor)
    }
}

impl Serialize for Rotor3 {
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        let mut state = serializer.serialize_struct("Rotor3", 2)?;
        state.serialize_field("s", &self.s)?;
        state.serialize_field("bv", &self.bv)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Rotor3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            S,
            Bv,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`s` or `bv`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "s" => Ok(Field::S),
                            "bv" => Ok(Field::Bv),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct Rotor3Visitor;

        impl<'de> Visitor<'de> for Rotor3Visitor {
            type Value = Rotor3;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Rotor3")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Rotor3, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let s = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let bv = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                Ok(Rotor3::new(s, bv))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Rotor3, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut s = None;
                let mut bv = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::S => {
                            if s.is_some() {
                                return Err(serde::de::Error::duplicate_field("s"));
                            }
                            s = Some(map.next_value()?);
                        }
                        Field::Bv => {
                            if bv.is_some() {
                                return Err(serde::de::Error::duplicate_field("bv"));
                            }
                            bv = Some(map.next_value()?);
                        }
                    }
                }
                let s = s.ok_or_else(|| serde::de::Error::missing_field("s"))?;
                let bv = bv.ok_or_else(|| serde::de::Error::missing_field("bv"))?;
                Ok(Rotor3::new(s, bv))
            }
        }

        const FIELDS: &[&str] = &["s", "bv"];

        deserializer.deserialize_struct("Rotor3", FIELDS, Rotor3Visitor)
    }
}

#[cfg(test)]
mod rotor_serde_tests {
    use crate::bivec::{Bivec2, Bivec3};
    use crate::rotor::{Rotor2, Rotor3};
    use serde_test::{assert_tokens, Token};

    #[test]
    fn bivec2() {
        let rotor2 = Rotor2::new(1., Bivec2::new(0.78));

        assert_tokens(
            &rotor2,
            &[
                Token::Struct {
                    name: "Rotor2",
                    len: 2,
                },
                Token::Str("s"),
                Token::F32(1.),
                Token::Str("bv"),
                Token::Struct {
                    name: "Bivec2",
                    len: 1,
                },
                Token::Str("xy"),
                Token::F32(0.78),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn rotor3() {
        let rotor3 = Rotor3::new(1., Bivec3::new(0.78, 0.36, 0.63));

        assert_tokens(
            &rotor3,
            &[
                Token::Struct {
                    name: "Rotor3",
                    len: 2,
                },
                Token::Str("s"),
                Token::F32(1.),
                Token::Str("bv"),
                Token::Struct {
                    name: "Bivec3",
                    len: 3,
                },
                Token::Str("xy"),
                Token::F32(0.78),
                Token::Str("xz"),
                Token::F32(0.36),
                Token::Str("yz"),
                Token::F32(0.63),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}

impl Serialize for Isometry2 {
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        let mut state = serializer.serialize_struct("Isometry2", 2)?;
        state.serialize_field("translation", &self.translation)?;
        state.serialize_field("rotation", &self.rotation)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Isometry2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Translation,
            Rotation,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`translation` or `rotation`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "translation" => Ok(Field::Translation),
                            "rotation" => Ok(Field::Rotation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct Isometry2Visitor;

        impl<'de> Visitor<'de> for Isometry2Visitor {
            type Value = Isometry2;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Isometry2")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Isometry2, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let translation = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let rotation = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                Ok(Isometry2::new(translation, rotation))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Isometry2, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut translation = None;
                let mut rotation = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Translation => {
                            if translation.is_some() {
                                return Err(serde::de::Error::duplicate_field("translation"));
                            }
                            translation = Some(map.next_value()?);
                        }
                        Field::Rotation => {
                            if rotation.is_some() {
                                return Err(serde::de::Error::duplicate_field("rotation"));
                            }
                            rotation = Some(map.next_value()?);
                        }
                    }
                }
                let translation =
                    translation.ok_or_else(|| serde::de::Error::missing_field("translation"))?;
                let rotation =
                    rotation.ok_or_else(|| serde::de::Error::missing_field("rotation"))?;
                Ok(Isometry2::new(translation, rotation))
            }
        }

        const FIELDS: &[&str] = &["rotation", "translation"];

        deserializer.deserialize_struct("Isometry2", FIELDS, Isometry2Visitor)
    }
}

impl Serialize for Isometry3 {
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        let mut state = serializer.serialize_struct("Isometry3", 2)?;
        state.serialize_field("translation", &self.translation)?;
        state.serialize_field("rotation", &self.rotation)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Isometry3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Translation,
            Rotation,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`translation` or `rotation`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "translation" => Ok(Field::Translation),
                            "rotation" => Ok(Field::Rotation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct Isometry3Visitor;

        impl<'de> Visitor<'de> for Isometry3Visitor {
            type Value = Isometry3;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Isometry3")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Isometry3, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let translation = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let rotation = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                Ok(Isometry3::new(translation, rotation))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Isometry3, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut translation = None;
                let mut rotation = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Translation => {
                            if translation.is_some() {
                                return Err(serde::de::Error::duplicate_field("translation"));
                            }
                            translation = Some(map.next_value()?);
                        }
                        Field::Rotation => {
                            if rotation.is_some() {
                                return Err(serde::de::Error::duplicate_field("rotation"));
                            }
                            rotation = Some(map.next_value()?);
                        }
                    }
                }
                let translation =
                    translation.ok_or_else(|| serde::de::Error::missing_field("translation"))?;
                let rotation =
                    rotation.ok_or_else(|| serde::de::Error::missing_field("rotation"))?;
                Ok(Isometry3::new(translation, rotation))
            }
        }

        const FIELDS: &[&str] = &["rotation", "translation"];

        deserializer.deserialize_struct("Isometry3", FIELDS, Isometry3Visitor)
    }
}
