//! ## `ultraviolet`
//! 
//! This is a crate to computer-graphics and games-related linear algebra, but *fast*, both in terms
//! of productivity and in terms of runtime performance.
//! 
//! In terms of productivity, ultraviolet uses no generics and is designed to be as straightforward
//! of an interface as possible, resulting in fast compilation times and clear code. In addition, the
//! lack of generics and Rust type-system "hacks" result in clear and concise errors that are easy to
//! parse and fix for the user.
//! 
//! In terms of runtime performance, ultraviolet was designed from the start with performance in mind.
//! To do so, we provide two separate kinds of each type, each with nearly identical functionality,
//! one with usual scalar f32 values, and the other a 'wide' type which uses SIMD f32x4 vectors for
//! each value. This design is clearn and explicit in intent, and it also allows code to
//! take full advantage of SIMD.
//! 
//! The 'wide' types use an "SoA" (Structure of Arrays) architecture
//! such that each `Wec` (wide-Vec) actually contains the data for 4 `Vec`s and will do any operation
//! on all 4 of the vector 'lanes' at the same time (the same concept applies to a `Wat`, or 'wide-Mat').
//! Doing this is potentially *much* (factor of 10)
//! faster than an "AoS" (Array of Structs) layout, as all current Rust linear algebra libraries do,
//! though it does depend on your workload. Algorithms must be carefully architected to take full advantage
//! of this, and doing so can be easier said than done, especially if your algorithm involves significant
//! branching.
//! 
//! ### Benchmarks
//! 
//! Benchmarks done using my own fork of [mathbench-rs](https://github.com/bitshifter/mathbench-rs) with support for
//! ultraviolet added to some benchmarks.
//! 
//! For the euler 2d and 3d benchmarks, the work being done is exactly equivalent. For the rest of the benchmarks,
//! the work being done is *made equivalent* by performing 4 of the benchmarked operation per iteration instead of just
//! one for all of the other libraries, since `ultraviolet` is computing that operation on four Vec/Mats at a time.
//! 
//! | benchmark              |        glam   |       cgmath   |     nalgebra   |       euclid   |   ultraviolet   |
//! |------------------------|---------------|----------------|----------------|----------------|-----------------|
//! | euler 2d               |    9.911 us   |     9.583 us   |     21.99 us   |     15.22 us   |    __6.675 us__ |
//! | euler 3d               |    15.11 us   |     32.88 us   |     237.2 us   |     32.62 us   |    __9.928 us__ |
//! | mat3 transform vector3 |   6.1533 ns   |   15.2933 ns   |   15.6202 ns   |      N/A       |   __4.4778 ns__ |
//! | vec3 cross             |   7.6824 ns   |   16.9919 ns   |   12.3683 ns   |   12.4657 ns   |   __3.3286 ns__ |
//! | vec3 dot               |   5.6354 ns   |   10.4704 ns   |    8.7803 ns   |    7.4304 ns   |   __2.4937 ns__ |
//! | vec3 length            |   5.8759 ns   |    4.2015 ns   |    4.5598 ns   |    4.2083 ns   |   __1.9067 ns__ |
//! | vec3 normalize         |   8.7861 ns   |    8.1677 ns   |   33.2839 ns   |    7.6300 ns   |   __4.4362 ns__ |
//! 
//! ### Features
//! 
//! This crate is currently being dogfooded in my ray tracer [`rayn`](https://github.com/termhn/rayn),
//! and is being used by some Amethyst developers in experimental projects while it is considered for adoption
//! into Amethyst. It does what those users have currently needed it to do.
//! 
//! There are a couple relatively unique/novel features in this lib, the most important being the use of the Geometric Algebra
//! concepts of Bivectors and Rotors to represent 2d and 3d rotations, rather than implementing complex number algebra
//! and Quaternion algebra.
//! 
//! What this means for the programmer is that you will be using the `Rotor3` type in place of
//! a Quaternion, though you can expect it to do basically all the same things that a Quaternion does. In fact, Quaternions
//! are essentially just a special case of Rotors. The reason this decision was made was twofold: first, the derivation of
//! the math is actually quite simple to understand. All the derivations for the code implemented in the Rotor structs in this
//! library are written out in the `docs` folder of the GitHub repo; I derived them manually as part of the implementation.
//! On the other hand, Quaternions are often basically just seen as black boxes that we programmers use to do rotations because
//! they have some nice properties, but that we don't really understand. You can use Rotors this same way, but you can also easily
//! understand them. Second is that in some sense they can be seen as 'more correct' than Quaternions, and especially they
//! facilitate a more proper understanding of rotation as being something that occurs *within a plane* rather than something
//! that occurs *around an axis*, as it is generally thought. Finally, Rotors also generalize do 4 and even higher dimensions,
//! and if someone wants to they could implement a Rotor4 which retains all the properties of a Rotor3/Quaternion but does rotation
//! in 4 dimensions instead, something which simply is not possible to do with Quaternions.
//! 
//! If it's missing something you need it to do, bug me on the [GitHub issue tracker](https://github.com/termhn/ultraviolet/issues) and/or Rust community discord server
//! (I'm Fusha there) and I'll try to add it for you, if I believe it fits with the vision of the lib :)

extern crate alloc;

mod util;

pub mod bivec;
pub mod lerp;
pub mod mat;
pub mod projection;
pub mod rotor;
pub mod transform;
pub mod vec;
pub mod int;
pub mod geometry;

pub use bivec::*;
pub use lerp::*;
pub use mat::*;
pub use rotor::*;
pub use transform::*;
pub use vec::*;
pub use int::*;

pub use wide;
pub use wide::f32x4;
