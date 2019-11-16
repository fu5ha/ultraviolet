//! ## `ultraviolet`
//!
//! This is a crate to do basic, computer-graphics-related, linear algebra, but *fast*, by
//! taking full advantage of SIMD. To do this, it uses an "SoA" (Structure of Arrays) architecture
//! such that each `Wec` (wide-vec) actually contains the data for 4 `Vec`s and will do any operation
//! on all 4 of the vector 'lanes' at the same time. Doing this is potentially *much* (factor of 10)
//! faster than an "AoS" (Array of Structs) layout, as all current Rust linear algebra libraries do.
//! However, algorithms must be carefully architected to take full advantage of this, and doing so
//! can be easier said than done, especially if your algorithm involves significant branching.
//!
//! This crate is currently being dogfooded in my ray tracer [`rayn`](https://github.com/termhn/rayn),
//! and it does what I need it to do.
//! If it's missing something you need it to do, bug me on the GitHub issue tracker and/or Rust
//! community discord server (I'm Fusha there) and I'll try to add it for you :)

extern crate alloc;

pub mod bivec;
pub mod lerp;
pub mod mat;
pub mod rotor;
mod util;
pub mod vec;

pub use bivec::*;
pub use lerp::*;
pub use mat::*;
pub use rotor::*;
pub use vec::*;

pub use wide;
pub use wide::f32x4;
