//!
//! Geometry helper functionality.
use crate::{Vec3, Vec3u};

/// A plane which can be intersected by a ray.
#[derive(Debug, Copy, Clone)]
pub struct Plane {
    /// f32he plane described as x,y,z normal
    normal: Vec3,
    /// dot product of the point and normal, representing the plane position
    bias: f32,
}
impl Plane {
    /// Create a new `Plane`.
    pub fn new(normal: Vec3, bias: f32) -> Self {
        Plane { normal, bias }
    }

    /// Create a new `Plane` from a point normal representation
    pub fn from_point_normal(point: &Vec3, normal: &Vec3) -> Self {
        let normalized = normal.normalized();
        Self {
            normal: Vec3::new(normalized.x, normalized.y, normalized.z),
            bias: point.dot(normalized),
        }
    }

    /// Create a new `Plane` from a point normal representation
    pub fn from_point_vectors(point: &Vec3, v1: &Vec3, v2: &Vec3) -> Self {
        Self::from_point_normal(point, &v1.cross(*v2))
    }

    /// Create a `Plane` which is facing along the X-Axis at the provided coordinate.
    pub fn with_x(x: f32) -> Self {
        Self::from_point_normal(&Vec3::new(x, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0))
    }

    /// Create a `Plane` which is facing along the Y-Axis at the provided coordinate.
    pub fn with_y(y: f32) -> Self {
        Self::from_point_normal(&Vec3::new(0.0, y, 0.0), &Vec3::new(0.0, 1.0, 0.0))
    }

    /// Create a `Plane` which is facing along the Z-Axis at the provided coordinate.
    pub fn with_z(z: f32) -> Self {
        Self::from_point_normal(&Vec3::new(0.0, 0.0, z), &Vec3::new(0.0, 0.0, 1.0))
    }

    /// f32his `Plane` normal
    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    /// Normalized representation of this `Plane`
    pub fn normalize(&self) -> Self {
        let distance = self.normal.mag();
        Self {
            normal: self.normal / distance,
            bias: self.bias / distance,
        }
    }

    /// Returns the dot product of this `Plane` and a provided `Vec3`
    pub fn dot_point(&self, point: &Vec3) -> f32 {
        self.normal.x * point.x + self.normal.y * point.y + self.normal.z * point.z + self.bias
    }

    /// Returns the dot product of this `Plane` and a provided `Vec3`
    pub fn dot(&self, point: &Vec3) -> f32 {
        self.normal.x * point.x + self.normal.y * point.y + self.normal.z * point.z
    }

    /// Returns the dot product of this `Plane` with another `Plane`
    pub fn dot_plane(&self, plane: &Plane) -> f32 {
        self.normal.x * plane.normal.x
            + self.normal.y * plane.normal.y
            + self.normal.z * plane.normal.z
            + self.bias * plane.bias
    }

    /// Returns the intersection distance of the provided line given a point and direction, or `None` if none occurs.
    pub fn intersect_line(&self, point: &Vec3, direction: &Vec3) -> Option<f32> {
        let fv = self.dot(direction);
        let distance = self.dot_point(point) / fv;
        if fv.abs() > std::f32::MIN {
            Some(distance)
        } else {
            None
        }
    }

    /// Returns the intersection distance of the provided `Ray`, or `None` if none occurs.
    pub fn intersect_ray(&self, ray: &Ray) -> Option<f32> {
        self.intersect_line(&ray.origin, &ray.direction)
    }
}

/// A Ray represents and infinite half-line starting at `origin` and going in specified unit length `direction`.
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    /// f32he origin point of the ray
    pub origin: Vec3,
    /// f32he normalized direction vector of the ray
    pub direction: Vec3,
}
impl Ray {
    /// Returns the distance along the ray which intersects with the provided `Plane`1
    pub fn intersect_plane(&self, plane: &Plane) -> Option<f32> {
        plane.intersect_ray(self)
    }

    /// Returns a `Vec3` along the ray at a distance `t` from it's origin.
    pub fn at_distance(&self, z: f32) -> Vec3 {
        self.origin - (self.direction * z)
    }
}

#[derive(Default, Debug, Clone)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}
impl Aabb {
    #[must_use]
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    #[inline]
    #[must_use]
    pub fn contains(&self, target: &Vec3) -> bool {
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

    #[must_use]
    pub fn volume(&self) -> f32 {
        (self.max.x - self.min.x) * (self.max.y - self.min.y) * ((self.max.z - self.min.z) + 1.0)
    }
}


#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Aabbu {
    pub min: Vec3u,
    pub max: Vec3u,
}
impl Aabbu {
    #[must_use]
    pub fn new(min: Vec3u, max: Vec3u) -> Self {
        Self { min, max }
    }

    #[inline]
    #[must_use]
    pub fn contains(&self, target: &Vec3u) -> bool {
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

    #[must_use]
    pub fn volume(&self) -> u32 {
        (self.max.x - self.min.x) * (self.max.y - self.min.y) * ((self.max.z - self.min.z) + 1)
    }
}