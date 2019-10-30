use std::ops::*;

use crate::vec::*;

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
