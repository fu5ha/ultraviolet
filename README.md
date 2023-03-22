[![crates.io](https://img.shields.io/crates/v/ultraviolet.svg)](https://crates.io/crates/ultraviolet)
[![docs.rs](https://docs.rs/ultraviolet/badge.svg)](https://docs.rs/ultraviolet)

# `ultraviolet`

This is a crate to computer-graphics and games-related linear and geometric algebra, but *fast*, both in terms
of productivity and in terms of runtime performance.

In terms of productivity, ultraviolet uses no generics and is designed to be as straightforward
of an interface as possible, resulting in fast compilation times and clear code. In addition, the
lack of generics and Rust type-system "hacks" result in clear and concise errors that are easy to
parse and fix for the user.

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

## Benchmarks

See [`mathbench-rs`](https://github.com/bitshifter/mathbench-rs) for latest benchmarks (may not be fully up-to-date with git master).

## Cargo Features

To help further improve build times, `ultraviolet` puts various functionality under feature flags. For example, the 2d and 3d projective geometric algebras
as well as f64 and integer types are disabled by default. In order to enable them, enable the corresponding crate feature flags in your `Cargo.toml`. For example:

```toml
[dependencies]
ultraviolet = { version = "0.9", features = [ "f64", "int" ] }
```

Will enable the `f64` and `int` features. Here's a list of the available features:

* `f64` – Enable `f64` bit wide floating point support. Naming convention is `D[Type]`, such as `DVec3x4` would be a collection of 4 3d vectors with `f64` precision each.
* `int` – Enable integer vector types.
* `bytemuck` – Enable casting of many types to byte arrays, for use with graphics APIs.
* `mint` – Enable interoperation with other math crates through the `mint` interface.
* `num-traits` – Enable [identity traits](https://docs.rs/num-traits/latest/num_traits/identities/index.html) for interoperation with other math crates.
* `serde` – Enable `Serialize` and `Deserialize` implementations for many scalar types.

## Crate Features

This crate is currently being dogfooded in my ray tracer [`rayn`](https://github.com/termhn/rayn),
and is being used by various independent Rust game developers for various projects.
It does what those users have currently needed it to do.

There are a couple relatively unique/novel features in this library, the most important being the use of the Geometric Algebra.

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

## Examples

### Euler Integration

[Euler Integration](https://en.wikipedia.org/wiki/Euler_method) is a method for numerically solving ordinary differential equations. If that sounds complicated, don't worry! The details of the method don't matter if you're not looking to implement any kind of physics simulation but this method is common in games. Keep reading for the code below!

The point is that if you are doing the same basic math operations on multiple floating point values with no conditionals (no `if`s), porting to wide data types and parallel processing is quite simple.

Here is the scalar example of Euler Integration:

```rust
fn integrate(
    pos: &mut [uv::Vec3],
    vel: &mut [uv::Vec3],
    acc: &[uv::Vec3],
    dt: f32,
) {
    for ((position, velocity), acceleration) in pos.iter_mut().zip(vel).zip(acc) {
        *velocity = *velocity + *acceleration * dt;
        *position = *position + *velocity * dt;
    }
}
```

The code loops over each set of corresponding position, velocity, and acceleration vectors. It first adjusts the velocity by the acceleration scaled by the amount of time that has passed and then adjusts the position by the velocity scaled by the amount of time that has passed.

These are all multiplication, addition, and assignment operators that need to be applied in the same way to all of the variables in question.

To port this function to wide data types and parallel processing, all we have to do is change the data types and we're done! The new function looks like this:

```rust
fn integrate_x8(
    pos: &mut [uv::Vec3x8],
    vel: &mut [uv::Vec3x8],
    acc: &[uv::Vec3x8],
    dt: f32x8,
) {
    for ((position, velocity), acceleration) in pos.iter_mut().zip(vel).zip(acc) {
        *velocity = *velocity + *acceleration * dt;
        *position = *position + *velocity * dt;
    }
}
```

This function now processes 8 sets of vectors in parallel and brings significant speed gains!

The only caveat is that the calling code that creates the slices of vectors needs to be modified to populate these wide data types with 8 sets of values instead of just one. The scalar code for that looks like this:

```rust
let mut pos: Vec<uv::Vec3> = Vec::with_capacity(100);
let mut vel: Vec<uv::Vec3> = Vec::with_capacity(100);
let mut acc: Vec<uv::Vec3> = Vec::with_capacity(100);

// You would probably write these constant values in-line but
// they are here for illustrative purposes
let pos_x = 1.0f32;
let pos_y = 2.0f32;
let pos_z = 3.0f32;

let vel_x = 4.0f32;
let vel_y = 5.0f32;
let vel_z = 6.0f32;

let acc_x = 7.0f32;
let acc_y = 8.0f32;
let acc_z = 9.0f32;

for ((position, velocity), acceleration) in pos.iter_mut().zip(vel).zip(acc) {
    pos.push(uv::Vec3::new(pos_x, pos_y, pos_z));
    vel.push(uv::Vec3::new(vel_x, vel_y, vel_z));
    acc.push(uv::Vec3::new(acc_x, acc_y, acc_z));
}
```

Whereas to populate the same for the 8-lane wide `Vec3x8` data type, the code could look like this:

```rust
let mut pos: Vec<uv::Vec3x8> = Vec::with_capacity(100 / 8 + 1);
let mut vel: Vec<uv::Vec3x8> = Vec::with_capacity(100 / 8 + 1);
let mut acc: Vec<uv::Vec3x8> = Vec::with_capacity(100 / 8 + 1);

let pos_x = uv::f32x8::splat(1.0f32);
let pos_y = uv::f32x8::splat(2.0f32);
let pos_z = uv::f32x8::splat(3.0f32);

let vel_x = uv::f32x8::splat(4.0f32);
let vel_y = uv::f32x8::splat(5.0f32);
let vel_z = uv::f32x8::splat(6.0f32);

let acc_x = uv::f32x8::splat(7.0f32);
let acc_y = uv::f32x8::splat(8.0f32);
let acc_z = uv::f32x8::splat(9.0f32);

for ((position, velocity), acceleration) in pos.iter_mut().zip(vel).zip(acc) {
    pos.push(uv::Vec3x8::new(pos_x, pos_y, pos_z));
    vel.push(uv::Vec3x8::new(vel_x, vel_y, vel_z));
    acc.push(uv::Vec3x8::new(acc_x, acc_y, acc_z));
}
```

Note that `100 / 8` in maths terms would be `12.5`, but we can't conveniently have a half-sized `Vec3x8`.

There are various ways to handle these 'remainder' vectors. You could fall back to scalar code, or progressively fall back to narrower wide types, such as `Vec3x4`, or you can just consider whether the cost of calculating a few additional vectors that you won't use is worth adding complexity to your code.

### Ray-Sphere Intersection

Scalar code that operates on a single value at a time needs some restructuring to take advantage of SIMD and the 4-/8-wide data types.

Below is an example of scalar ray-sphere instersection code using `Vec3` for points and vectors:

```rust
fn ray_sphere_intersect(
    ray_o: uv::Vec3,
    ray_d: uv::Vec3,
    sphere_o: uv::Vec3,
    sphere_r_sq: f32,
) -> f32 {
    let oc = ray_o - sphere_o;
    let b = oc.dot(ray_d);
    let c = oc.mag_sq() - sphere_r_sq;
    let descrim = b * b - c;

    if descrim > 0.0 {
        let desc_sqrt = descrim.sqrt();

        let t1 = -b - desc_sqrt;
        if t1 > 0.0 {
            t1
        } else {
            let t2 = -b + desc_sqrt;
            if t2 > 0.0 {
                t2
            } else {
                f32::MAX
            }
        }
    } else {
        f32::MAX
    }
}
```

This porting guide will not discuss the details of the algorithm, but will focus on how to convert the code to apply parallel SIMD operations on wide data types.

The first thing to do is to convert the parameter and return types from scalar `Vec3` to wide `Vec3x8` and `f32x8`:

```rust
fn ray_sphere_intersect_x8(
    ray_o: uv::Vec3x8,
    ray_d: uv::Vec3x8,
    sphere_o: uv::Vec3x8,
    sphere_r_sq: uv::f32x8,
) -> uv::f32x8 {
```

Each call to the function will process 8 ray-sphere intersections in parallel. The first four lines of the function remain the same:

```rust
    let oc = ray_o - sphere_o;
    let b = oc.dot(ray_d);
    let c = oc.mag_sq() - sphere_r_sq;
    let descrim = b * b - c;
```

Despite this code being the same, the calculations for 8 rays and spheres will be carried out at the same time!

The next line of the scalar code tests the value of `descrim` to see if it is greater than `0.0`. When operating on 8 values at a time, the code cannot branch along two separate paths because the value of `descrim` for each of the 8 values may cause branching to different sets of operations. To support this we would need to convert back to scalar code and then we lose all the performance benefits of parallel processing.

So, how do we convert this? We have a tradeoff to consider depending on the frequency of divergence, that is depending on how often the branch will follow one or the other path. If it is very likely for the given data and algorithm that the majority of branches will take one path, we can check whether all lanes take that path and then branch based on that. Such a bias toward one branch path is relatively rare, and in the case of this algorithm it is common to branch either way so this approach would produce slower code.

Another approach is to calculate the results for both branches for all 8 lanes, and then filter the results with masks that select the correct values from the possibilities at the end.

To create the mask for 8 lanes of `descrim` values with `0.0`:

```rust
    let desc_pos = descrim.cmp_gt(uv::f32x8::splat(0.0));
```

In the true case of the original scalar version, we then have more arithmetic operations that end up looking the exact same when we do them on the vectorized version:

```rust
    let desc_sqrt = descrim.sqrt();

    let t1 = -b - desc_sqrt;
```

And now in the scalar code we have another branch based on `t1 > 0.0`, so we apply the same technique, with a little bit extra:

```rust
    let t1_valid = t1.cmp_gt(uv::f32x8::splat(0.0)) & desc_pos;
```

The `& desc_pos` at the end does a bitwise and operation to combine the masks that say whether each of the lanes of `t1 > 0.0` are true or false, with those of whether each of the lanes of `descrim > 0.0` were true or false, and if both are true for a lane, then the mask value will be true for that lane in `t1_mask`, otherwise the value for the lane will be `false`. This is combining the nested logic.

The true case of the `t1 > 0.0` condition just returns `t1`, but the false case has some more calculation and branching that can be ported in a similar way:

```rust
    let t2 = -b + desc_sqrt;
    let t2_valid = t2.cmp_gt(uv::f32x8::splat(0.0)) & desc_pos;
```

This may sound like it could be slower than scalar code because this algorithm being applied to wide data types is doing all the calculations for both branches regardless of which is true, and you would be right!

This approach is indeed a tradeoff and depends on the likelihood of branching one way or the other, and the cost of calculation of the branches. However, even with an algorithm that is particularly branch-heavy like the ray-sphere intersection we're analyzing here, in practice, the benefits of being able to calculate multiple pieces of data simultaneously often results in a net win! As with all optimization, measurement tells the truth.

At this point, we have ported almost the entire algorithm. We have values for `t1` and `t2` for each of the 8 lanes. We have mask values in `t1_valid` that indicate whether both `descrim > 0.0 && t1 > 0.0` for each lane. And we have `t2_valid` with values indicating exactly `descrim > 0.0 && t2 > 0.0`. When the scalar code does not return `t1` or `t2`, it returns `f32::MAX`. How do we now select the correct return value for each of the lanes?

`ultraviolet` has a `blend` function on the mask types that uses the true or false values for each of the lanes to select from the calculated values for the true and false cases. So if `a` were a wide vector of values that would be calculated in the true case of a branch, and `b` were for the false case, with a mask `m` we could select from `a` and `b` based on `m` by calling `m.blend(a, b)` and the result would be the desired output values!

Let's try to apply that to the scalar code by looking just at its logical control flow:

```rust
    if descrim > 0.0 {
        if t1 > 0.0 {
            t1
        } else {
            if t2 > 0.0 {
                t2
            } else {
                f32::MAX
            }
        }
    } else {
        f32::MAX
    }
```

So if we take the outer-most if condition..

```rust
   let t = t1_valid.blend(t1, ???);
```

What is the value for false case of the `descrim > 0.0 && t1 > 0.0` test? There are two possibilities - either `descrim <= 0.0`, which is the false case of the `descrim > 0.0` condition, or `descrim > 0.0 && t1 <= 0.0` which is the else case where we handle `t2`. This looks complicated. Let's try looking at the `descrim > 0.0 && t2 > 0.0` case in the scalar code and try `blend`ing that:

```rust
    let t = t2_valid.blend(t2, uv::f32x8::splat(std::f32::MAX));
```

So `descrim > 0.0 && t2 > 0.0` has two false cases, either `descrim <= 0.0` and we want to return `f32::MAX`, or `descrim > 0.0 && t2 <= 0.0` and we want to return `f32::MAX`, so we can `blend` to select the correct values here to cover the false case of the scalar `descrim > 0.0` condition, and the false case of the `t1 > 0.0` condition, that leaves only the true case of the `t1 > 0.0` condition left to resolve...

And that is exactly what `t1_valid.blend(t1, ???)` would select! So we can combine the two blends like this:

```rust
    let t = t2_valid.blend(t2, uv::f32x8::splat(std::f32::MAX));
    let t = t1_valid.blend(t1, t);
```

`t` now contains `t1`, `t2` or `f32::MAX` as appropriate for each of the lanes! We have completed the port of the scalar algorithm code to leverage SIMD operations on 8-lane wide data types to calculate 8 ray-sphere intersections in parallel!

Below is the full example of the same ray-sphere intersection algorithm implemented using the wide `Vec3x8` type:

```rust
fn ray_sphere_intersect_x8(
    sphere_o: uv::Vec3x8,
    sphere_r_sq: uv::f32x8,
    ray_o: uv::Vec3x8,
    ray_d: uv::Vec3x8,
) -> uv::f32x8 {
    let oc = ray_o - sphere_o;
    let b = oc.dot(ray_d);
    let c = oc.mag_sq() - sphere_r_sq;
    let descrim = b * b - c;

    let desc_pos = descrim.cmp_gt(uv::f32x8::splat(0.0));

    let desc_sqrt = descrim.sqrt();

    let t1 = -b - desc_sqrt;
    let t1_valid = t1.cmp_gt(uv::f32x8::splat(0.0)) & desc_pos;

    let t2 = -b + desc_sqrt;
    let t2_valid = t2.cmp_gt(uv::f32x8::splat(0.0)) & desc_pos;

    let t = t2_valid.blend(t2, uv::f32x8::splat(std::f32::MAX));
    let t = t1_valid.blend(t1, t);

    t
}
```
