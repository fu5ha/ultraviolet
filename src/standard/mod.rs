//! Standard linear algebra primitives based on 2d/3d/4d vector spaces plus geometric algebra in R2 & R3
//! 
//! Contains standard linear algebra primitives in 2d/3d/4d vector spaces such as vectors and euclidean transformation types.
//!
//! Also contains some geometric algebra types in the geometric algebra spaces R(2,0,0) and R(3,0,0) (the standard 2d and 3d vector spaces)
//! mixed in.

pub mod bivec;
#[cfg(feature = "geometry")]
pub mod geometry;
pub mod rotor;
pub mod transform;
pub mod vec;

pub use bivec::*;
pub use rotor::*;
pub use transform::*;
pub use vec::*;
