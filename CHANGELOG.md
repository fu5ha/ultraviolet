## 0.6
- Upgrade `wide` to 0.5.x
- Rename `[WideType]::merge()` to `[WideType]::blend()`
- Add `wgpu`-specfic notes to `projection` module (adds `_wgpu` to some function names)
- Add support for `packed_simd` under "nightly" feature flag (required nightly Rust compiler)
- Under nightly, add support for 256-bit and 512-bit wide SIMD vectors
- Add support for f64/double precision floats
- Add spherical linear interpolation and better docs around interpolation
- Add `Into<Vec2; N> for Vec2xN` implementations