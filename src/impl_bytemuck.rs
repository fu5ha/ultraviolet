use crate::*;

use bytemuck::{Pod, Zeroable};

// ...

unsafe impl Pod for Vec2 {}
unsafe impl Zeroable for Vec2 {}

unsafe impl Pod for Vec3 {}
unsafe impl Zeroable for Vec3 {}

unsafe impl Pod for Vec4 {}
unsafe impl Zeroable for Vec4 {}

unsafe impl Pod for Bivec2 {}
unsafe impl Zeroable for Bivec2 {}

unsafe impl Pod for Bivec3 {}
unsafe impl Zeroable for Bivec3 {}

unsafe impl Pod for Rotor2 {}
unsafe impl Zeroable for Rotor2 {}

unsafe impl Pod for Rotor3 {}
unsafe impl Zeroable for Rotor3 {}

unsafe impl Pod for Mat2 {}
unsafe impl Zeroable for Mat2 {}

unsafe impl Pod for Mat3 {}
unsafe impl Zeroable for Mat3 {}

unsafe impl Pod for Mat4 {}
unsafe impl Zeroable for Mat4 {}

unsafe impl Pod for Isometry2 {}
unsafe impl Zeroable for Isometry2 {}

unsafe impl Pod for Isometry3 {}
unsafe impl Zeroable for Isometry3 {}

unsafe impl Pod for Similarity2 {}
unsafe impl Zeroable for Similarity2 {}

unsafe impl Pod for Similarity3 {}
unsafe impl Zeroable for Similarity3 {}

// ...

#[cfg(feature = "f64")]
unsafe impl Pod for DVec2 {}
#[cfg(feature = "f64")]
unsafe impl Zeroable for DVec2 {}

#[cfg(feature = "f64")]
unsafe impl Pod for DVec3 {}
#[cfg(feature = "f64")]
unsafe impl Zeroable for DVec3 {}

#[cfg(feature = "f64")]
unsafe impl Pod for DVec4 {}
#[cfg(feature = "f64")]
unsafe impl Zeroable for DVec4 {}

#[cfg(feature = "f64")]
unsafe impl Pod for DBivec2 {}
#[cfg(feature = "f64")]
unsafe impl Zeroable for DBivec2 {}

#[cfg(feature = "f64")]
unsafe impl Pod for DBivec3 {}
#[cfg(feature = "f64")]
unsafe impl Zeroable for DBivec3 {}

#[cfg(feature = "f64")]
unsafe impl Pod for DRotor2 {}
#[cfg(feature = "f64")]
unsafe impl Zeroable for DRotor2 {}

#[cfg(feature = "f64")]
unsafe impl Pod for DRotor3 {}
#[cfg(feature = "f64")]
unsafe impl Zeroable for DRotor3 {}

#[cfg(feature = "f64")]
unsafe impl Pod for DMat2 {}
#[cfg(feature = "f64")]
unsafe impl Zeroable for DMat2 {}

#[cfg(feature = "f64")]
unsafe impl Pod for DMat3 {}
#[cfg(feature = "f64")]
unsafe impl Zeroable for DMat3 {}

#[cfg(feature = "f64")]
unsafe impl Pod for DMat4 {}
#[cfg(feature = "f64")]
unsafe impl Zeroable for DMat4 {}

#[cfg(feature = "f64")]
unsafe impl Pod for DIsometry2 {}
#[cfg(feature = "f64")]
unsafe impl Zeroable for DIsometry2 {}

#[cfg(feature = "f64")]
unsafe impl Pod for DIsometry3 {}
#[cfg(feature = "f64")]
unsafe impl Zeroable for DIsometry3 {}

#[cfg(feature = "f64")]
unsafe impl Pod for DSimilarity2 {}
#[cfg(feature = "f64")]
unsafe impl Zeroable for DSimilarity2 {}

#[cfg(feature = "f64")]
unsafe impl Pod for DSimilarity3 {}
#[cfg(feature = "f64")]
unsafe impl Zeroable for DSimilarity3 {}

// ...

#[cfg(feature = "int")]
unsafe impl Pod for IVec2 {}
#[cfg(feature = "int")]
unsafe impl Zeroable for IVec2 {}

#[cfg(feature = "int")]
unsafe impl Pod for IVec3 {}
#[cfg(feature = "int")]
unsafe impl Zeroable for IVec3 {}

#[cfg(feature = "int")]
unsafe impl Pod for IVec4 {}
#[cfg(feature = "int")]
unsafe impl Zeroable for IVec4 {}

#[cfg(feature = "int")]
unsafe impl Pod for UVec2 {}
#[cfg(feature = "int")]
unsafe impl Zeroable for UVec2 {}

#[cfg(feature = "int")]
unsafe impl Pod for UVec3 {}
#[cfg(feature = "int")]
unsafe impl Zeroable for UVec3 {}

#[cfg(feature = "int")]
unsafe impl Pod for UVec4 {}
#[cfg(feature = "int")]
unsafe impl Zeroable for UVec4 {}
