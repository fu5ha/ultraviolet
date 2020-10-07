use crate::*;

use bytemuck::{Pod, Zeroable};

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
