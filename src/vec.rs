use std::ops::*;

use wide::f32x4;

macro_rules! vec2s {
    ($($n:ident => $t:ident),+) => {
        $(#[derive(Clone, Copy, Debug)]
        pub struct $n {
            pub x: $t,
            pub y: $t,
        }

        impl $n {
            #[inline]
            pub fn new<T: Into<$t>>(x: T, y: T) -> Self {
                $n { x: x.into(), y: y.into() }
            }

            #[inline]
            pub fn broadcast<T: Into<$t> + Copy>(val: T) -> Self {
                Self::new(val, val)
            }

            #[inline]
            pub fn dot(&self, other: $n) -> $t {
                self.x * other.x + self.y * other.y
            }

            #[inline]
            pub fn reflect(&mut self, normal: $n) {
                *self = *self - ($t::from(2.0) * self.dot(normal) * normal);
            }

            #[inline]
            pub fn reflected(&self, normal: $n) -> Self {
                let mut a = *self;
                a.reflect(normal);
                a
            }

            #[inline]
            pub fn mag_sq(&self) -> $t {
                self.x * self.x + self.y * self.y
            }

            #[inline]
            pub fn mag(&self) -> $t {
                self.mag_sq().sqrt()
            }

            #[inline]
            pub fn normalize(&mut self) {
                let mag = self.mag();
                self.x /= mag;
                self.y /= mag;
            }

            #[inline]
            pub fn normalized(&self) -> Self {
                let mut r = self.clone();
                r.normalize();
                r
            }

            #[inline]
            pub fn mul_add(&self, mul: $n, add: $n) -> Self {
                $n::new(
                    self.x.mul_add(mul.x, add.x),
                    self.y.mul_add(mul.x, add.x),
                )
            }

            #[inline]
            pub fn map<F>(&self, f: F) -> Self
                where F: Fn($t) -> $t
            {
                $n::new(
                    f(self.x),
                    f(self.y),
                )
            }

            #[inline]
            pub fn apply<F>(&mut self, f: F)
                where F: Fn($t) -> $t
            {
                self.x = f(self.x);
                self.y = f(self.y);
            }

            #[inline]
            pub fn component_max(&self) -> $t {
                self.x.max(self.y)
            }

            #[inline]
            pub fn component_min(&self) -> $t {
                self.x.min(self.y)
            }

            #[inline]
            pub fn zero() -> Self {
                Self::broadcast($t::from(0.0))
            }

            #[inline]
            pub fn one() -> Self {
                Self::broadcast($t::from(1.0))
            }
        }

        impl Add for $n {
            type Output = Self;
            #[inline]
            fn add(self, rhs: $n) -> Self {
                $n::new(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl AddAssign for $n {
            #[inline]
            fn add_assign(&mut self, rhs: $n) {
                self.x += rhs.x;
                self.y += rhs.y;
            }
        }

        impl Sub for $n {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: $n) -> Self {
                $n::new(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl SubAssign for $n {
            #[inline]
            fn sub_assign(&mut self, rhs: $n) {
                self.x -= rhs.x;
                self.y -= rhs.y;
            }
        }

        impl Mul for $n {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: $n) -> Self {
                $n::new(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<$n> for $t {
            type Output = $n;
            #[inline]
            fn mul(self, rhs: $n) -> $n {
                $n::new(self * rhs.x, self * rhs.y)
            }
        }

        impl Mul<$t> for $n {
            type Output = $n;
            #[inline]
            fn mul(self, rhs: $t) -> $n {
                $n::new(self.x * rhs, self.y * rhs)
            }
        }

        impl MulAssign for $n {
            #[inline]
            fn mul_assign(&mut self, rhs: $n) {
                self.x *= rhs.x;
                self.y *= rhs.y;
            }
        }

        impl MulAssign<$t> for $n {
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                self.x *= rhs;
                self.y *= rhs;
            }
        }

        impl Div for $n {
            type Output = Self;
            #[inline]
            fn div(self, rhs: $n) -> Self {
                $n::new(self.x / rhs.x, self.y / rhs.y)
            }
        }

        impl Div<$t> for $n {
            type Output = $n;
            #[inline]
            fn div(self, rhs: $t) -> $n {
                $n::new(self.x / rhs, self.y / rhs)
            }
        }

        impl DivAssign for $n {
            #[inline]
            fn div_assign(&mut self, rhs: $n) {
                self.x /= rhs.x;
                self.y /= rhs.y;
            }
        }

        impl DivAssign<$t> for $n {
            #[inline]
            fn div_assign(&mut self, rhs: $t) {
                self.x /= rhs;
                self.y /= rhs;
            }
        }

        impl Neg for $n {
            type Output = $n;
            #[inline]
            fn neg(self) -> $n {
                self * $t::from(-1.0)
            }
        })+
    };
}

vec2s!(Vec2 => f32, Wec2 => f32x4);

impl From<[Vec2; 4]> for Wec2 {
    #[inline]
    fn from(vecs: [Vec2; 4]) -> Self {
        Self {
            x: f32x4::new(vecs[0].x, vecs[1].x, vecs[2].x, vecs[3].x),
            y: f32x4::new(vecs[0].y, vecs[1].y, vecs[2].y, vecs[3].y),
        }
    }
}

impl Wec2 {
    #[inline]
    pub fn splat(vec: Vec2) -> Self {
        Self::from([vec, vec, vec, vec])
    }

    #[inline]
    pub fn merge(mask: f32x4, a: Self, b: Self) -> Self {
        Self {
            x: f32x4::merge(mask, a.x, b.x),
            y: f32x4::merge(mask, a.y, b.y),
        }
    }
}

macro_rules! vec3s {
    ($($n:ident => $t:ident),+) => {
        $(#[derive(Clone, Copy, Debug)]
        pub struct $n {
            pub x: $t,
            pub y: $t,
            pub z: $t,
        }

        impl $n {
            #[inline]
            pub fn new<T: Into<$t>>(x: T, y: T, z: T) -> Self {
                $n { x: x.into(), y: y.into(), z: z.into() }
            }

            #[inline]
            pub fn broadcast<T: Into<$t> + Copy>(val: T) -> Self {
                Self::new(val, val, val)
            }

            #[inline]
            pub fn unit_x() -> Self {
                $n{ x: $t::from(1.0), y: $t::from(0.0), z: $t::from(0.0) }
            }

            #[inline]
            pub fn unit_y() -> Self {
                $n{ x: $t::from(0.0), y: $t::from(1.0), z: $t::from(0.0) }
            }

            #[inline]
            pub fn unit_z() -> Self {
                $n{ x: $t::from(0.0), y: $t::from(0.0), z: $t::from(1.0) }
            }

            #[inline]
            pub fn dot(&self, other: $n) -> $t {
                self.x * other.x + self.y * other.y + self.z * other.z
            }

            #[inline]
            pub fn cross(&self, other: $n) -> Self {
                $n::new(
                    self.y * other.z - self.z * other.y,
                    self.z * other.x - self.x * other.z,
                    self.x * other.y - self.y * other.x,
                )
            }

            #[inline]
            pub fn reflect(&mut self, normal: $n) {
                *self = *self - ($t::from(2.0) * self.dot(normal) * normal);
            }

            #[inline]
            pub fn reflected(&self, normal: $n) -> Self {
                let mut a = *self;
                a.reflect(normal);
                a
            }

            #[inline]
            pub fn mag_sq(&self) -> $t {
                self.x * self.x + self.y * self.y + self.z * self.z
            }

            #[inline]
            pub fn mag(&self) -> $t {
                self.mag_sq().sqrt()
            }

            #[inline]
            pub fn normalize(&mut self) {
                let mag = self.mag();
                self.x /= mag;
                self.y /= mag;
                self.z /= mag;
            }

            #[inline]
            pub fn normalized(&self) -> Self {
                let mut r = self.clone();
                r.normalize();
                r
            }

            #[inline]
            pub fn mul_add(&self, mul: $n, add: $n) -> Self {
                $n::new(
                    self.x.mul_add(mul.x, add.x),
                    self.y.mul_add(mul.y, add.y),
                    self.z.mul_add(mul.z, add.z),
                )
            }

            #[inline]
            pub fn map<F>(&self, f: F) -> Self
                where F: Fn($t) -> $t
            {
                $n::new(
                    f(self.x),
                    f(self.y),
                    f(self.z)
                )
            }

            #[inline]
            pub fn apply<F>(&mut self, f: F)
                where F: Fn($t) -> $t
            {
                self.x = f(self.x);
                self.y = f(self.y);
                self.z = f(self.z);
            }

            #[inline]
            pub fn component_max(&self) -> $t {
                self.x.max(self.y).max(self.z)
            }

            #[inline]
            pub fn component_min(&self) -> $t {
                self.x.min(self.y).min(self.z)
            }

            #[inline]
            pub fn zero() -> Self {
                Self::broadcast($t::from(0.0))
            }

            #[inline]
            pub fn one() -> Self {
                Self::broadcast($t::from(1.0))
            }
        }

        impl From<[$t; 3]> for $n {
            #[inline]
            fn from(comps: [$t; 3]) -> Self {
                Self::new(comps[0], comps[1], comps[2])
            }
        }

        impl Add for $n {
            type Output = Self;
            #[inline]
            fn add(self, rhs: $n) -> Self {
                $n::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
            }
        }

        impl AddAssign for $n {
            #[inline]
            fn add_assign(&mut self, rhs: $n) {
                self.x += rhs.x;
                self.y += rhs.y;
                self.z += rhs.z;
            }
        }

        impl Sub for $n {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: $n) -> Self {
                $n::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
            }
        }

        impl SubAssign for $n {
            #[inline]
            fn sub_assign(&mut self, rhs: $n) {
                self.x -= rhs.x;
                self.y -= rhs.y;
                self.z -= rhs.z;
            }
        }

        impl Mul for $n {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: $n) -> Self {
                $n::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
            }
        }

        impl Mul<$n> for $t {
            type Output = $n;
            #[inline]
            fn mul(self, rhs: $n) -> $n {
                $n::new(self * rhs.x, self * rhs.y, self * rhs.z)
            }
        }

        impl Mul<$t> for $n {
            type Output = $n;
            #[inline]
            fn mul(self, rhs: $t) -> $n {
                $n::new(self.x * rhs, self.y * rhs, self.z * rhs)
            }
        }

        impl MulAssign for $n {
            #[inline]
            fn mul_assign(&mut self, rhs: $n) {
                self.x *= rhs.x;
                self.y *= rhs.y;
                self.z *= rhs.z;
            }
        }

        impl MulAssign<$t> for $n {
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                self.x *= rhs;
                self.y *= rhs;
                self.z *= rhs;
            }
        }

        impl Div for $n {
            type Output = Self;
            #[inline]
            fn div(self, rhs: $n) -> Self {
                $n::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
            }
        }

        impl Div<$t> for $n {
            type Output = $n;
            #[inline]
            fn div(self, rhs: $t) -> $n {
                $n::new(self.x / rhs, self.y / rhs, self.z / rhs)
            }
        }

        impl DivAssign for $n {
            #[inline]
            fn div_assign(&mut self, rhs: $n) {
                self.x /= rhs.x;
                self.y /= rhs.y;
                self.z /= rhs.z;
            }
        }

        impl DivAssign<$t> for $n {
            #[inline]
            fn div_assign(&mut self, rhs: $t) {
                self.x /= rhs;
                self.y /= rhs;
                self.z /= rhs;
            }
        }

        impl Neg for $n {
            type Output = $n;
            #[inline]
            fn neg(self) -> $n {
                self * $t::from(-1.0)
            }
        })+
    }
}

vec3s!(Vec3 => f32, Wec3 => f32x4);

impl From<Vec2> for Vec3 {
    #[inline]
    fn from(vec: Vec2) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: f32::from(0.0),
        }
    }
}

impl From<Wec2> for Wec3 {
    #[inline]
    fn from(vec: Wec2) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: f32x4::from(0.0),
        }
    }
}

impl Wec3 {
    #[inline]
    pub fn splat(vec: Vec3) -> Self {
        Self::from([vec, vec, vec, vec])
    }

    #[inline]
    pub fn merge(mask: f32x4, a: Self, b: Self) -> Self {
        Self {
            x: f32x4::merge(mask, a.x, b.x),
            y: f32x4::merge(mask, a.y, b.y),
            z: f32x4::merge(mask, a.z, b.z),
        }
    }
}

impl Into<[Vec3; 4]> for Wec3 {
    #[inline]
    fn into(self) -> [Vec3; 4] {
        let xs = self.x.as_ref();
        let ys = self.y.as_ref();
        let zs = self.z.as_ref();
        [
            Vec3::new(xs[0], ys[0], zs[0]),
            Vec3::new(xs[1], ys[1], zs[1]),
            Vec3::new(xs[2], ys[2], zs[2]),
            Vec3::new(xs[3], ys[3], zs[3]),
        ]
    }
}

impl From<[Vec3; 4]> for Wec3 {
    #[inline]
    fn from(vecs: [Vec3; 4]) -> Self {
        Self {
            x: f32x4::new(vecs[0].x, vecs[1].x, vecs[2].x, vecs[3].x),
            y: f32x4::new(vecs[0].y, vecs[1].y, vecs[2].y, vecs[3].y),
            z: f32x4::new(vecs[0].z, vecs[1].z, vecs[2].z, vecs[3].z),
        }
    }
}
