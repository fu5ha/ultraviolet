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
            pub fn new(x: $t, y: $t) -> Self {
                $n { x, y }
            }

            pub fn dot(&self, other: $n) -> $t {
                self.x * other.x + self.y * other.y
            }

            pub fn mag_sq(&self) -> $t {
                self.x * self.x + self.y * self.y
            }

            pub fn mag(&self) -> $t {
                self.mag_sq().sqrt()
            }

            pub fn normalize(&mut self) {
                let mag = self.mag();
                self.x /= mag;
                self.y /= mag;
            }

            pub fn normalized(&self) -> Self {
                let mut r = self.clone();
                r.normalize();
                r
            }

            pub fn mul_add(&self, mul: $n, add: $n) -> Self {
                $n::new(
                    self.x.mul_add(mul.x, add.x),
                    self.y.mul_add(mul.x, add.x),
                )
            }
        }

        impl Add for $n {
            type Output = Self;
            fn add(self, rhs: $n) -> Self {
                $n::new(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl Sub for $n {
            type Output = Self;
            fn sub(self, rhs: $n) -> Self {
                $n::new(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl Mul for $n {
            type Output = Self;
            fn mul(self, rhs: $n) -> Self {
                $n::new(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<$n> for $t {
            type Output = $n;
            fn mul(self, rhs: $n) -> $n {
                $n::new(self * rhs.x, self * rhs.y)
            }
        }

        impl Mul<$t> for $n {
            type Output = $n;
            fn mul(self, rhs: $t) -> $n {
                $n::new(self.x * rhs, self.y * rhs)
            }
        }

        impl Div for $n {
            type Output = Self;
            fn div(self, rhs: $n) -> Self {
                $n::new(self.x / rhs.x, self.y / rhs.y)
            }
        }

        impl Div<$t> for $n {
            type Output = $n;
            fn div(self, rhs: $t) -> $n {
                $n::new(self.x / rhs, self.y / rhs)
            }
        }

        impl Neg for $n {
            type Output = $n;
            fn neg(self) -> $n {
                self * $t::from(-1.0)
            }
        })+
    };
}

vec2s!(Vec2 => f32, Wec2 => f32x4);

impl From<[Vec2; 4]> for Wec2 {
    fn from(vecs: [Vec2; 4]) -> Self {
        Self {
            x: f32x4::new(vecs[0].x, vecs[1].x, vecs[2].x, vecs[3].x),
            y: f32x4::new(vecs[0].y, vecs[1].y, vecs[2].y, vecs[3].y),
        }
    }
}

impl Wec2 {
    pub fn splat(vec: Vec2) -> Self {
        Self::from([vec, vec, vec, vec])
    }

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
            pub fn new(x: $t, y: $t, z: $t) -> Self {
                $n { x, y, z }
            }

            pub fn unit_x() -> Self {
                $n{ x: $t::from(1.0), y: $t::from(0.0), z: $t::from(0.0) }
            }

            pub fn unit_y() -> Self {
                $n{ x: $t::from(0.0), y: $t::from(1.0), z: $t::from(0.0) }
            }

            pub fn unit_z() -> Self {
                $n{ x: $t::from(0.0), y: $t::from(0.0), z: $t::from(1.0) }
            }

            pub fn dot(&self, other: $n) -> $t {
                self.x * other.x + self.y * other.y + self.z * other.z
            }

            pub fn cross(&self, other: $n) -> Self {
                $n::new(
                    self.y * other.z - self.z * other.y,
                    self.z * other.x - self.x * other.z,
                    self.x * other.y - self.y * other.x,
                )
            }

            pub fn reflected(&self, normal: $n) -> Self {
                *self - ($t::from(2.0) * self.dot(normal) * normal)
            }

            pub fn mag_sq(&self) -> $t {
                self.x * self.x + self.y * self.y + self.z * self.z
            }

            pub fn mag(&self) -> $t {
                self.mag_sq().sqrt()
            }

            pub fn normalize(&mut self) {
                let mag = self.mag();
                self.x /= mag;
                self.y /= mag;
                self.z /= mag;
            }

            pub fn normalized(&self) -> Self {
                let mut r = self.clone();
                r.normalize();
                r
            }

            pub fn mul_add(&self, mul: $n, add: $n) -> Self {
                $n::new(
                    self.x.mul_add(mul.x, add.x),
                    self.y.mul_add(mul.y, add.y),
                    self.z.mul_add(mul.z, add.z),
                )
            }

            pub fn map<F>(&self, f: F) -> Self
                where F: Fn($t) -> $t
            {
                $n::new(
                    f(self.x),
                    f(self.y),
                    f(self.z)
                )
            }

            pub fn apply<F>(&mut self, f: F)
                where F: Fn($t) -> $t
            {
                self.x = f(self.x);
                self.y = f(self.y);
                self.z = f(self.z);
            }

            pub fn component_max(&self) -> $t {
                self.x.max(self.y).max(self.z)
            }

            pub fn component_min(&self) -> $t {
                self.x.min(self.y).min(self.z)
            }
        }

        impl From<[$t; 3]> for $n {
            fn from(comps: [$t; 3]) -> Self {
                Self::new(comps[0], comps[1], comps[2])
            }
        }

        impl Add for $n {
            type Output = Self;
            fn add(self, rhs: $n) -> Self {
                $n::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
            }
        }

        impl AddAssign for $n {
            fn add_assign(&mut self, rhs: $n) {
                self.x += rhs.x;
                self.y += rhs.y;
                self.z += rhs.z;
            }
        }

        impl Sub for $n {
            type Output = Self;
            fn sub(self, rhs: $n) -> Self {
                $n::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
            }
        }

        impl SubAssign for $n {
            fn sub_assign(&mut self, rhs: $n) {
                self.x -= rhs.x;
                self.y -= rhs.y;
                self.z -= rhs.z;
            }
        }

        impl Mul for $n {
            type Output = Self;
            fn mul(self, rhs: $n) -> Self {
                $n::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
            }
        }

        impl Mul<$n> for $t {
            type Output = $n;
            fn mul(self, rhs: $n) -> $n {
                $n::new(self * rhs.x, self * rhs.y, self * rhs.z)
            }
        }

        impl Mul<$t> for $n {
            type Output = $n;
            fn mul(self, rhs: $t) -> $n {
                $n::new(self.x * rhs, self.y * rhs, self.z * rhs)
            }
        }

        impl MulAssign for $n {
            fn mul_assign(&mut self, rhs: $n) {
                self.x *= rhs.x;
                self.y *= rhs.y;
                self.z *= rhs.z;
            }
        }

        impl MulAssign<$t> for $n {
            fn mul_assign(&mut self, rhs: $t) {
                self.x *= rhs;
                self.y *= rhs;
                self.z *= rhs;
            }
        }

        impl Div for $n {
            type Output = Self;
            fn div(self, rhs: $n) -> Self {
                $n::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
            }
        }

        impl Div<$t> for $n {
            type Output = $n;
            fn div(self, rhs: $t) -> $n {
                $n::new(self.x / rhs, self.y / rhs, self.z / rhs)
            }
        }

        impl DivAssign for $n {
            fn div_assign(&mut self, rhs: $n) {
                self.x /= rhs.x;
                self.y /= rhs.y;
                self.z /= rhs.z;
            }
        }

        impl DivAssign<$t> for $n {
            fn div_assign(&mut self, rhs: $t) {
                self.x /= rhs;
                self.y /= rhs;
                self.z /= rhs;
            }
        }

        impl Neg for $n {
            type Output = $n;
            fn neg(self) -> $n {
                self * $t::from(-1.0)
            }
        })+
    }
}

vec3s!(Vec3 => f32, Wec3 => f32x4);

impl Vec3 {
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
}

impl From<Vec2> for Vec3 {
    fn from(vec: Vec2) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: f32::from(0.0),
        }
    }
}

impl From<Wec2> for Wec3 {
    fn from(vec: Wec2) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: f32x4::from(0.0),
        }
    }
}

impl Wec3 {
    pub fn zero() -> Self {
        Self::new(f32x4::from(0.0), f32x4::from(0.0), f32x4::from(0.0))
    }

    pub fn one() -> Self {
        Self::new(f32x4::from(1.0), f32x4::from(1.0), f32x4::from(1.0))
    }

    pub fn splat(vec: Vec3) -> Self {
        Self::from([vec, vec, vec, vec])
    }

    pub fn merge(mask: f32x4, a: Self, b: Self) -> Self {
        Self {
            x: f32x4::merge(mask, a.x, b.x),
            y: f32x4::merge(mask, a.y, b.y),
            z: f32x4::merge(mask, a.z, b.z),
        }
    }
}

impl Into<[Vec3; 4]> for Wec3 {
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
    fn from(vecs: [Vec3; 4]) -> Self {
        Self {
            x: f32x4::new(vecs[0].x, vecs[1].x, vecs[2].x, vecs[3].x),
            y: f32x4::new(vecs[0].y, vecs[1].y, vecs[2].y, vecs[3].y),
            z: f32x4::new(vecs[0].z, vecs[1].z, vecs[2].z, vecs[3].z),
        }
    }
}

macro_rules! mat3s {
    ($($n:ident => $t:ident),+) => {
        $(#[derive(Clone, Copy, Debug)]
        pub struct $n {
            pub cols: [$t; 3],
        }

        impl $n {
            pub fn new(col1: $t, col2: $t, col3: $t) -> Self {
                $n {
                    cols: [col1, col2, col3],
                }
            }
        }

        impl Mul<$t> for $n {
            type Output = $t;
            fn mul(self, rhs: $t) -> $t {
                let a = self.cols[0];
                let b = self.cols[1];
                let c = self.cols[2];
                $t::new(
                    a.x * rhs.x + b.x * rhs.y + c.x * rhs.z,
                    a.y * rhs.x + b.y * rhs.y + c.y * rhs.z,
                    a.z * rhs.x + b.z * rhs.y + c.z * rhs.z,
                )
            }
        })+
    }
}

mat3s!(Mat3 => Vec3, Wat3 => Wec3);
