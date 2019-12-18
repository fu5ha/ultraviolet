//!
//! Geometry helper functionality.
use crate::{Vec3, Vec3u, Vec3i};

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
#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

/// An axis-aligned bounding box
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Aabbu {
    pub min: Vec3u,
    pub max: Vec3u,
}

/// An axis-aligned bounding box
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Aabbi {
    pub min: Vec3i,
    pub max: Vec3i,
}

macro_rules! impl_aabb {
    ($($n:ident, $v3t:ident => $t:ident),+) => {
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
            pub fn volume(&self) -> $t {
                (self.max.x - self.min.x) * (self.max.y - self.min.y) * ((self.max.z - self.min.z) + 1 as $t)
            }
        })+
    }
}

impl_aabb!(Aabb, Vec3 => f32, Aabbu, Vec3u => u32, Aabbi, Vec3i => i32);