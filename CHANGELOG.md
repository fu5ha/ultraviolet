<!-- next-header -->
## Unreleased
- Add Mat3 into Rotor3 conversion for rotation matrices
- Remove heavy reliance on `mul_add` due to negligible performance benefit and in many cases performance detriment.
- Slightly optimize Vector `normalize`.
- `Rotor2::from_angle_plane()` now takes plane and angle as separate arguments.

## 0.6.1
- Add scalar multiplication and componentwise addition for `MatN`

## 0.6
- Significantly improve performance of Rotors and transform types (Isometry, Similarity)
- Add `Rotor3::rotate_vecs()` for improved performance on rotating multiple vecs with the same rotor
- Add support for f64/double precision floats under `f64` feature. Naming convention is `D[TypeName]` for the f64 versions.
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
