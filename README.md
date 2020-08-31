[![crates.io](http://meritbadge.herokuapp.com/ultraviolet)](https://crates.io/crates/ultraviolet)
[![docs.rs](https://docs.rs/ultraviolet/badge.svg)](https://docs.rs/ultraviolet)

## `ultraviolet`

This is a crate to computer-graphics and games-related linear and geometric algebra, but *fast*, both in terms
of productivity and in terms of runtime performance.

In terms of productivity, ultraviolet uses no generics and is designed to be as straightforward
of an interface as possible, resulting in fast compilation times and clear code. In addition, the
lack of generics and Rust type-system "hacks" result in clear and concise errors that are easy to
parse and fix for the user. It also implements optimized versions of the extremely expressive 2d
and 3d projective geometric algebras P(R(2,0,1)) and P(R(3,0,1)) which make working with geometric primitives
in 2d and 3d euclidean space extremely intuitive. Check out the 
[Siggraph 2019 course on Geometric Algebra](https://www.youtube.com/watch?v=tX4H_ctggYo/) for a great
introduction to the topic, and and the final chapter of Eric Lengyel's
[Foundations of Game Engine Development, Volume 1: Mathematics](https://www.amazon.com/Foundations-Game-Engine-Development-Mathematics/dp/0985811749)
for a more thorough and grounded introduction to its practical use.

In terms of runtime performance, ultraviolet was designed from the start with performance in mind.
To do so, we provide two separate kinds of each type, each with nearly identical functionality,
one with usual scalar f32 values, and the other a 'wide' type which uses SIMD f32x4 vectors for
each value. This design is clear and explicit in intent, and it also allows code to
take full advantage of SIMD.

The 'wide' types use an "SoA" (Structure of Arrays) architecture
such that each wide data structure actually contains the data for 4 or 8 of its associated data type and will do any operation
on all of the simd 'lanes' at the same time. For example, a `Vec3x8` is equivalent to 8 `Vec3`s all bundled together into one
data structure.

Doing this is potentially *much* (factor of 10) faster than an standard "AoS" (Array of Structs) layout,
though it does depend on your workload and algorithm requirements. Algorithms must be carefully architected to take full advantage
of this, and doing so can be easier said than done, especially if your algorithm involves significant branching.

`ultraviolet` was the first Rust math library to be designed in this "AoSoA" manner, though
`nalgebra` now supports it for several of their data structures as well.

#### Cargo Features

To help further improve build times, `ultraviolet` puts various functionality under feature flags. For example, the 2d and 3d projective geometric algebras
as well as f64 and integer types are disabled by default. In order to enable them, enable the corresponding crate feature flags in your `Cargo.toml`. For example:

```toml
[dependencies]
ultraviolet = { version = "0.7", features = [ "f64", "pga3d" ] }
```

Will enable the `f64` and `pga3d` features. Here's a list of the available features:

* `f64` - Enable f64 bit wide floating point support. Naming convention is `D[Type]`, such as `DVec3x4` would be a collection of 4 3d vectors with f64 precision each.
* `int` - Enable integer vector types.
* `pga2d`/`pga3d` - Enable the 2d and 3d Projective Geometric Algebra modules, respectively
* `geometry` - Enable some geometry helper functionality, adding structures such as `Ray`, `Aabb`, etc.

### Features

This crate is currently being dogfooded in my ray tracer [`rayn`](https://github.com/termhn/rayn),
and is being used by various independent Rust game developers for various projects.
It does what those users have currently needed it to do.

There are a couple relatively unique/novel features in this library, the most important being the use of the Geometric Algebra.
This library implements 3d and 2d Projective Geometric Algebra in the `pga3d` and `pga2d` modules, but it also uses some geometric algebra
within the mostly linear-algebra `standard` module.

Instead of implementing complex number algebra (for 2d rotations) and Quaternion algebra (for 3d rotations), we use
Rotors, a concept taken from Geometric Algebra, to represent 2d and 3d rotations.

What this means for the programmer is that you will be using the `Rotor3` type in place of
a Quaternion, though you can expect it to do basically all the same things that a Quaternion does. In fact, Quaternions
are directly isomorphic to Rotors (meaning they are in essense the same thing, just formulated differently). The reason this decision was made was twofold:
first, the derivation of the math is actually quite simple to understand. All the derivations for the code implemented in the Rotor structs in this
library are written out in the `derivations` folder of the GitHub repo; I derived them manually as part of the implementation.

On the other hand, Quaternions are often basically just seen as black boxes that we programmers use to do rotations because
they have some nice properties, but that we don't really understand. You can use Rotors this same way, but you can also easily
understand them. Second is that in some sense they can be seen as 'more correct' than Quaternions. Specifically, they
facilitate a more proper understanding of rotation as being something that occurs *within a plane* rather than something
that occurs *around an axis*, as it is generally thought. Finally, Rotors also generalize to 4 and even higher dimensions,
and if someone wants to they could implement a Rotor4 which retains all the properties of a Rotor3/Quaternion but does rotation
in 4 dimensions instead, something which simply is not possible to do with Quaternions.

If it's missing something you need it to do, bug me on the [GitHub issue tracker](https://github.com/termhn/ultraviolet/issues) and/or Rust community discord server
(I'm Fusha there) and I'll try to add it for you, if I believe it fits with the vision of the lib :)
