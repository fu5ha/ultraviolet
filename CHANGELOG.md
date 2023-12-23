# Changelog

<!-- next-header -->
## Unreleased

- Update `projection` module's and submodules' documentation
- Remove `rh_ydown` projection module as it was very unlikely to be useful and caused confusion.
- Add `mint` type conversions for integer vectors
- Implement Serialize and Deserialize for `Similarity`
- Implement Serialize and Deserialize for f64 types: `DBivec`, `DRotor`, `DIsometry`, `DSimilarity`
- Add type conversion between `mint` quaternion and `Rotor3`

## 0.9.2

- Add `num_traits::identities` support behind a `num-traits` feature flag
- Fix inverse for similarities
- Fix missing `mut` in `Mat4::as_mut_array` result

## 0.9.1

- Fix `from_euler_angles` on `Mat4`
- Update documentation on matrix rotation constructors
- Update some documentation and improve implementation on
`as_array` methods on various types

## 0.9.0

- Add methods to scale `Rotor3`
- Implement `Neg` for all `IVec` types
- Relax bound in `map` and `apply` from `Fn` to `FnMut`
- Make matrix serialization and deserialization implementations match
- Update `wide` to `0.7.x`

## 0.8.1

- Implement Serialize and Deserialize for all `UVec`,`IVec`, `DVec`, `DMat` types
  (under `serde`, `int` and `f64` feature flags)
- Implement conversions between integer and float vectors.
- Add `#[must_use]` attributes on `.normalized` methods to help prevent silent logic bugs when
`.normalize()` would have been more appropriate.
- Build docs.rs docs with all features.

## 0.8.0

- Update `wide` to `0.6.x`
- Add `Rotor3::into_angle_plane()`
- Add `Rotor3::into_quaternion_array` and `Rotor3::from_quaternion_array`
- Implement Serialize and Deserialize for `Isometry2` and `Isometry3` (under `serde` feature flag)
- Added `const` to `new` functions for integer vectors.
- Add `Mat4::truncate()`
- Add `Mul<Scalar>` and `Add<Self>` for isometries and similarities

## 0.7.5

- Add `Mat4::extract_translation`, `Mat4::extract_rotation` and `Mat4::into_isometry`.
- Add missing `PartialEq` implementations for all matrices, transformations, vectors, bivectors and rotors, including
  SIMD and `f64` variants
- Fix `Rotor2::rotate_vec` and corresponding derivation.

## 0.7.4

- Add optional bytemuck support

## 0.7.3

- Fix integer types not compiling properly.

## 0.7.2

- Implement Serialize and Deserialize for bivectors and rotors (under `serde` feature flag)

## 0.7.1

- Fix typo in `Mat3::inverse` implementation which made it transpose instead

## 0.7.0

- Add Mat3 into Rotor3 conversion for rotation matrices
- Remove heavy reliance on `mul_add` due to negligible performance benefit and in many cases performance detriment.
- Slightly optimize Vector `normalize`.
- `Rotor2::from_angle_plane()` now takes plane and angle as separate arguments.
- Add `MatN::adjugate()`.
- `Mat3::from_nonuniform_scale_homogeneous()` now takes a `Vec2` instead of a `Vec3`.

## 0.6.1

- Add scalar multiplication and componentwise addition for `MatN`

## 0.6

- Significantly improve performance of Rotors and transform types (Isometry, Similarity)
- Add `Rotor3::rotate_vecs()` for improved performance on rotating multiple vecs with the same rotor
- Add support for f64/double precision floats under `f64` feature. Naming convention is `D[TypeName]` for the f64
  versions.
- Rename `W[TypeName]` to `[TypeName]x4`, allowing room for `[TypeName]x8`.
- Add support for 256 bit AVX vectors.
- Add support for `mint` for scalar types
- Add `wgpu`-specfic notes to `projection` module (adds `_wgpu` to some function names)
- Add spherical linear interpolation and better docs around interpolation
- Rename `[WideType]::merge()` to `[WideType]::blend()`
- Add `Into<Vec2; N> for Vec2xN` implementations
- Fix some doc comments not appearing properly on Vec and Mat types.
- Make most initializers `const`
- Various performance improvements, especially for Rotor-transform-vector and some matrix operations
- Add `MatN::determinant()`
- Add `Mat2::inverse()`
