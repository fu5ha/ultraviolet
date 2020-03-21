//!
//! Geometry helper functionality.
use crate::{Vec3, Vec3i, Vec3u};

/// A plane which can be intersected by a ray.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Plane {
    /// plane described as x,y,z normal
    pub normal: Vec3,

    /// dot product of the point and normal, representing the plane position
    pub bias: f32,
}

/// A Ray represents an infinite half-line starting at `origin` and going in specified unit length `direction`.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Ray {
    /// origin point of the ray
    pub origin: Vec3,

    /// normalized direction vector of the ray
    pub direction: Vec3,
}

/// A plane which can be intersected by a ray.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Planeu {
    /// plane described as x,y,z normal
    pub normal: Vec3u,

    /// dot product of the point and normal, representing the plane position
    pub bias: u32,
}

/// A Ray represents an infinite half-line starting at `origin` and going in specified unit length `direction`.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Rayu {
    /// origin point of the ray
    pub origin: Vec3u,

    /// normalized direction vector of the ray
    pub direction: Vec3u,
}

/// A plane which can be intersected by a ray.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Planei {
    /// plane described as x,y,z normal
    pub normal: Vec3i,

    /// dot product of the point and normal, representing the plane position
    pub bias: i32,
}

/// A Ray represents an infinite half-line starting at `origin` and going in specified unit length `direction`.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Rayi {
    /// origin point of the ray
    pub origin: Vec3i,

    /// normalized direction vector of the ray
    pub direction: Vec3i,
}

macro_rules! impl_plane_ray {
    ($($pn:ident, $rn:ident, $v3t:ident => $t:ident),+) => {
        $(
            impl $rn {
                /// Returns the distance along the ray which intersects with the provided `Plane`
                #[inline]
                pub fn intersect_plane(&self, plane: $pn) -> Option<$t> {
                    plane.intersect_ray(*self)
                }

                /// Returns a `Vec3` along the ray at a distance `t` from it's origin.
                #[inline]
                pub fn at_distance(&self, z: $t) -> $v3t {
                    self.direction.mul_add($v3t::broadcast(z), self.origin)
                }
            }

            impl $pn {
                /// Create a new `Plane`.
                #[inline]
                pub fn new(normal: $v3t, bias: $t) -> Self {
                    $pn { normal, bias }
                }

                /// Create a new `Plane` from a point normal representation. The normal parameter must already be normalized.
                #[inline]
                pub fn from_point_normal(point: $v3t, normal: $v3t) -> Self {
                    Self {
                        normal,
                        bias: point.dot(normal),
                    }
                }

                /// Create a new `Plane` from a point normal representation
                #[inline]
                pub fn from_point_vectors(point: $v3t, v1: $v3t, v2: $v3t) -> Self {
                    Self::from_point_normal(point, v1.cross(v2))
                }

                /// Create a `Plane` which is facing along the X-Axis at the provided coordinate.
                #[inline]
                pub fn with_x(x: $t) -> Self {
                    Self::from_point_normal($v3t::new(x, 0 as $t, 0 as $t), $v3t::new(1 as $t, 0 as $t, 0 as $t,))
                }

                /// Create a `Plane` which is facing along the Y-Axis at the provided coordinate.
                #[inline]
                pub fn with_y(y: $t) -> Self {
                    Self::from_point_normal($v3t::new(0 as $t, y, 0 as $t), $v3t::new(0 as $t, 1 as $t, 0 as $t))
                }

                /// Create a `Plane` which is facing along the Z-Axis at the provided coordinate.
                #[inline]
                pub fn with_z(z: $t) -> Self {
                    Self::from_point_normal($v3t::new(0 as $t, 0 as $t, z), $v3t::new(0 as $t, 0 as $t, 1 as $t))
                }

                /// f32his `Plane` normal
                #[inline]
                pub fn normal(&self) -> $v3t {
                    self.normal
                }

                /// Normalized representation of this `Plane`
                #[inline]
                pub fn normalize(&mut self)  {
                    let distance = self.normal.mag();
                    self.normal /= distance;
                    self.bias /= distance;
                }

                /// Normalized representation of this `Plane`
                #[inline]
                pub fn normalized(&self) -> Self {
                    let distance = self.normal.mag();
                    Self {
                        normal: self.normal / distance,
                        bias: self.bias / distance,
                    }
                }

                /// Returns the dot product of this `Plane` and a provided `Vec3`
                #[inline]
                pub fn dot_point(&self, point: $v3t) -> $t {
                    self.normal.x * point.x + self.normal.y * point.y + self.normal.z * point.z + self.bias
                }

                /// Returns the dot product of this `Plane` and a provided `Vec3`, assumed to be a normal, computed with this planes normal.
                #[inline]
                pub fn dot(&self, point: $v3t) -> $t {
                    self.normal.x * point.x + self.normal.y * point.y + self.normal.z * point.z
                }

                /// Returns the dot product of this `Plane` with another `Plane`. This is computed against the two plane normals.
                #[inline]
                pub fn dot_plane(&self, plane: $pn) -> $t {
                    self.normal.x * plane.normal.x
                        + self.normal.y * plane.normal.y
                        + self.normal.z * plane.normal.z
                        + self.bias * plane.bias
                }

                /// Returns the intersection distance of the provided line given a point and direction, or `None` if none occurs.
                ///
                /// Warning: These intersection methods do not check for the ray never intersecting. This is up to the user to confirm.
                #[inline]
                pub fn intersect_line(&self, point: $v3t, direction: $v3t) -> Option<$t> {
                    let fv = self.dot(direction);
                    let distance = self.dot_point(point) / fv;

                    Some(distance)
                }

                /// Returns the intersection distance of the provided `Ray`, or `None` if none occurs.
                ///
                /// Warning: These intersection methods do not check for the ray never intersecting. This is up to the user to confirm.
                #[inline]
                pub fn intersect_ray(&self, ray: $rn) -> Option<$t> {
                    self.intersect_line(ray.origin, ray.direction)
                }
            }
        )+
    }
}

impl_plane_ray!(Plane, Ray, Vec3 => f32);
impl_plane_ray!(Planeu, Rayu, Vec3u => u32);
impl_plane_ray!(Planei, Rayi, Vec3i => i32);

/// An axis-aligned bounding box
#[derive(Default, Debug, Copy, Clone)]
#[repr(C)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

/// An axis-aligned bounding box
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Aabbu {
    pub min: Vec3u,
    pub max: Vec3u,
}

/// An axis-aligned bounding box
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Aabbi {
    pub min: Vec3i,
    pub max: Vec3i,
}

macro_rules! impl_aabb {
    ($($n:ident, $iter:ident, $v3t:ident => $t:ident),+) => {
        $(
        impl $n {
            /// Creates a new axis-aligned bounding box.
            ///
            /// `min` **must** be less than or equal to `max`. This is not checked by the library, but will result in
            /// bad results and/or unsigned integer underflow if it is not held.
            #[must_use]
            pub fn new(min: $v3t, max: $v3t) -> Self {
                Self { min, max }
            }

            #[inline]
            #[must_use]
            pub fn contains(&self, target: $v3t) -> bool {
                target.x >= self.min.x
                    && target.x <= self.max.x
                    && target.y >= self.min.y
                    && target.y <= self.max.y
                    && target.z >= self.min.z
                    && target.z <= self.max.z
            }

            #[inline]
            #[must_use]
            pub fn intersects(&self, other: &Self) -> bool {
                (self.min.x <= other.max.x && self.max.x >= other.min.x)
                    && (self.min.y <= other.max.y && self.max.y >= other.min.y)
                    && (self.min.z <= other.max.z && self.max.z >= other.min.z)
            }

            #[inline]
            #[must_use]
            pub fn size(&self) -> $v3t {
                self.max - self.min
            }

            #[inline]
            #[must_use]
            pub fn volume(&self) -> $t {
                self.size().x * self.size().y * self.size().z
            }

            /// Returns the smallest bounding box that contains both this bounding box and the other bounding box.
            #[inline]
            #[must_use]
            pub fn join(&self, other: &Self) -> Self {
                Self::new(
                    $v3t::new(self.min.x.min(other.min.x),
                        self.min.y.min(other.min.y),
                        self.min.z.min(other.min.z)),
                    $v3t::new(self.max.x.max(other.max.x),
                        self.max.y.max(other.max.y),
                        self.max.z.max(other.max.z)
                    )
                )
            }

            /// Returns the smallest bounding box that contains both this bounding box and a point.
            #[inline]
            #[must_use]
            pub fn grow(&self, other: &$v3t) -> Self {
                Self::new(
                    $v3t::new(
                        self.min.x.min(other.x),
                        self.min.y.min(other.y),
                        self.min.z.min(other.z),
                    ),
                    $v3t::new(
                        self.max.x.max(other.x),
                        self.max.y.max(other.y),
                        self.max.z.max(other.z),
                    ),
                )
            }

            #[inline]
            #[must_use]
            pub fn largest_axis(&self) -> usize {
                if self.size().x > self.size().y && self.size().x > self.size().z {
                    0
                } else if self.size().y > self.size().z {
                    1
                } else {
                    2
                }
            }

            #[inline]
            #[must_use]
            pub fn iter_stride(&self, stride: $t) -> $iter {
                $iter::new(*self, stride)
            }
        }

        /// Linear iterator across a 3D coordinate space with the provided stride.
        /// This iterator is inclusive of minimum coordinates, and exclusive of maximum.
        pub struct $iter {
            stride: $t,
            track: $v3t,
            region: $n,
        }
        impl $iter {
            /// Create a new iterator.
            #[must_use]
            pub fn new(region: $n, stride: $t) -> Self {
                Self {
                    track: region.min,
                    region,
                    stride,
                }
            }
        }
        impl Iterator for $iter {
            type Item = $v3t;

            fn next(&mut self) -> Option<Self::Item> {
                let ret = self.track;

                if self.track.z >= self.region.max.z {
                    return None;
                }

                if self.track.x >= self.region.max.x - (1 as $t) {
                    self.track.y += self.stride;
                    self.track.x = self.region.min.x;
                } else {
                    self.track.x += self.stride;
                    return Some(ret);
                }

                if self.track.y >= self.region.max.y {
                    self.track.z += self.stride;

                    self.track.y = self.region.min.y;
                }

                Some(ret)
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                let cur_volume = ($n::new(self.track, self.region.max).volume() / self.stride / self.stride / self.stride) as usize;
                let volume = (self.region.volume() / self.stride / self.stride / self.stride) as usize;
                (volume - cur_volume, Some(volume))
            }
        }
        impl ExactSizeIterator for $iter {}
        )+
    }
}

impl Aabb {
    /// Same as iter_stride, but calls it with a stride of 1.0
    #[inline]
    #[must_use]
    pub fn iter(&self) -> AabbLinearIterator {
        self.iter_stride(1.0)
    }

    #[inline]
    #[must_use]
    pub fn surface_area(&self) -> f32 {
        2.0 * (self.size().x * self.size().y + self.size().x * self.size().z + self.size().y * self.size().z)
    }

    #[inline]
    #[must_use]
    pub fn center(&self) -> Vec3 {
        self.min + (self.size() / 2.0)
    }
}

impl Aabbu {
    /// Same as iter_stride, but calls it with a stride of 1.0
    #[inline]
    #[must_use]
    pub fn iter(&self) -> AabbuLinearIterator {
        self.iter_stride(1)
    }

    #[inline]
    #[must_use]
    pub fn surface_area(&self) -> u32 {
        2 * (self.size().x * self.size().y + self.size().x * self.size().z + self.size().y * self.size().z)
    }

    #[inline]
    #[must_use]
    pub fn center(&self) -> Vec3u {
        self.min + (self.size() / 2)
    }
}

impl Aabbi {
    /// Same as iter_stride, but calls it with a stride of 1.0
    #[inline]
    #[must_use]
    pub fn iter(&self) -> AabbiLinearIterator {
        self.iter_stride(1)
    }

    #[inline]
    #[must_use]
    pub fn surface_area(&self) -> i32 {
        2 * (self.size().x * self.size().y + self.size().x * self.size().z + self.size().y * self.size().z)
    }

    #[inline]
    #[must_use]
    pub fn center(&self) -> Vec3i {
        self.min + (self.size() / 2)
    }
}

impl_aabb!(Aabb, AabbLinearIterator, Vec3 => f32, Aabbu, AabbuLinearIterator, Vec3u => u32, Aabbi, AabbiLinearIterator, Vec3i => i32);
