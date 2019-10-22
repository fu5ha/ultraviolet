use std::ops::*;

use wide::f32x4;

macro_rules! vec2s {
    ($($n:ident => $t:ident),+) => {
        $(#[derive(Clone, Copy)]
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
        })+
    };
}

vec2s!(Vec2 => f32, Wec2 => f32x4);

macro_rules! vec3s {
    ($($n:ident => $t:ident),+) => {
        $(#[derive(Clone, Copy)]
        pub struct $n {
            pub x: $t,
            pub y: $t,
            pub z: $t,
        }

        impl $n {
            pub fn new(x: $t, y: $t, z: $t) -> Self {
                $n { x, y, z }
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
                    self.y.mul_add(mul.x, add.x),
                    self.z.mul_add(mul.x, add.x),
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

        impl Add for $n {
            type Output = Self;
            fn add(self, rhs: $n) -> Self {
                $n::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
            }
        }

        impl Sub for $n {
            type Output = Self;
            fn sub(self, rhs: $n) -> Self {
                $n::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
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

impl Wec3 {
    pub fn zero() -> Self {
        Self::new(f32x4::from(0.0), f32x4::from(0.0), f32x4::from(0.0))
    }

    pub fn one() -> Self {
        Self::new(f32x4::from(1.0), f32x4::from(1.0), f32x4::from(1.0))
    }

    pub fn from_vecs(v1: Vec3, v2: Vec3, v3: Vec3, v4: Vec3) -> Self {
        Self {
            x: f32x4::new(v1.x, v2.x, v3.x, v4.x),
            y: f32x4::new(v1.y, v2.y, v3.y, v4.y),
            z: f32x4::new(v1.z, v2.z, v3.z, v4.z),
        }
    }

    pub fn to_vecs(self) -> [Vec3; 4] {
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

macro_rules! mat3s {
    ($($n:ident => $t:ident),+) => {
        $(#[derive(Clone, Copy)]
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
                    a.x * rhs.x + a.y * rhs.x + a.z * rhs.x,
                    b.x * rhs.y + b.y * rhs.y + b.z * rhs.y,
                    c.x * rhs.z + c.y * rhs.z + c.z * rhs.z,
                )
            }
        })+
    }
}

mat3s!(Mat3 => Vec3, Wat3 => Wec3);
