#[repr(C)]
#[derive(Default, Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Vec3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl Vec3i {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub const fn xy(&self) -> Vec2i {
        Vec2i::new(self.x, self.y)
    }
}

impl From<(i32, i32, i32)> for Vec3i {
    fn from(other: (i32, i32, i32)) -> Self {
        Self {
            x: other.0,
            y: other.1,
            z: other.2,
        }
    }
}

impl ::core::ops::AddAssign for Vec3i {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Vec3u {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
impl Vec3u {
    pub const fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    pub const fn xy(&self) -> Vec2u {
        Vec2u::new(self.x, self.y)
    }
}

impl From<(u32, u32, u32)> for Vec3u {
    fn from(other: (u32, u32, u32)) -> Self {
        Self {
            x: other.0,
            y: other.1,
            z: other.2,
        }
    }
}

impl ::core::ops::AddAssign for Vec3u {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[repr(C)]
#[derive(Default, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}
impl Vec2i {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<(i32, i32)> for Vec2i {
    fn from(other: (i32, i32)) -> Self {
        Self {
            x: other.0,
            y: other.1,
        }
    }
}

impl ::core::ops::AddAssign for Vec2i {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[repr(C)]
#[derive(Default, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Vec2u {
    pub x: u32,
    pub y: u32,
}
impl Vec2u {
    pub const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl From<(u32, u32)> for Vec2u {
    fn from(other: (u32, u32)) -> Self {
        Self {
            x: other.0,
            y: other.1,
        }
    }
}

impl From<[u32; 2]> for Vec2u {
    fn from(other: [u32; 2]) -> Self {
        Self {
            x: other[0],
            y: other[1],
        }
    }
}

impl ::core::ops::AddAssign for Vec2u {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
