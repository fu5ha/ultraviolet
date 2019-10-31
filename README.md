## `ultraviolet`

This is a crate to do basic, computer-graphics-related, linear algebra, but *fast*, by
taking full advantage of SIMD. To do this, it uses an "SoA" (Structure of Arrays) architecture
such that each `Wec` (wide-vec) actually contains the data for 4 `Vec`s and will do any operation
on all 4 of the vector 'lanes' at the same time. Doing this is potentially *much* (factor of 10)
faster than an "AoS" (Array of Structs) layout, as all current Rust linear algebra libraries do,
depending on your work load. However, algorithms must be carefully architected to take full advantage
of this, and doing so can be easier said than done, especially if your algorithm involves significant
branching.

### Benchmarks

Benchmarks done using my own fork of [mathbench-rs](https://github.com/bitshifter/mathbench-rs) with support for
ultraviolet added to some benchmarks.

For the euler 2d and 3d benchmarks, the work being done is exactly equivalent. For the rest of the benchmarks,
the work being done is *made equivalent* by performing 4 of the benchmarked operation per iteration instead of just
one for all of the other libraries, since `ultraviolet` is computing that operation on four Vec/Mats at a time.

| benchmark              |        glam   |       cgmath   |     nalgebra   |       euclid   |   ultraviolet   |
|------------------------|---------------|----------------|----------------|----------------|-----------------|
| euler 2d               |    9.911 us   |     9.583 us   |     21.99 us   |     15.22 us   |    __6.675 us__ |
| euler 3d               |    15.11 us   |     32.88 us   |     237.2 us   |     32.62 us   |    __9.928 us__ |
| mat3 transform vector3 |   6.1533 ns   |   15.2933 ns   |   15.6202 ns   |      N/A       |   __4.4778 ns__ |
| vec3 cross             |   7.6824 ns   |   16.9919 ns   |   12.3683 ns   |   12.4657 ns   |   __3.3286 ns__ |
| vec3 dot               |   5.6354 ns   |   10.4704 ns   |    8.7803 ns   |    7.4304 ns   |   __2.4937 ns__ |
| vec3 length            |   5.8759 ns   |    4.2015 ns   |    4.5598 ns   |    4.2083 ns   |   __1.9067 ns__ |
| vec3 normalize         |   8.7861 ns   |    8.1677 ns   |   33.2839 ns   |    7.6300 ns   |   __4.4362 ns__ |

### Features

This crate is currently being dogfooded in my ray tracer [`rayn`](https://github.com/termhn/rayn),
and it does what I need it to do. If it's missing something you need it to do, bug me on the GitHub
issue tracker and/or Rust community discord server (I'm Fusha there) and I'll try to add it for you :)