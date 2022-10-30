use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub normal: Vec3,
    pub vertices: [Vec3; 3]
}

#[derive(Clone, Copy, Debug)]
pub enum Line3 {
    Vec {
        origin: Vec3,
        end: Vec3
    },
    Parameterized {
        origin: Vec3,
        direction: Vec3
    }
}

impl Vec3 {
    pub fn new(data: [f32; 3]) -> Vec3 {
        Vec3 {
            x: data[0],
            y: data[1],
            z: data[2]
        }
    }

    pub fn as_arr(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(other: [f32; 3]) -> Vec3 {
        Vec3::new(other)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }
}

impl Line3 {
    pub fn vertex_form(origin: Vec3, end: Vec3) -> Self {
        Line3::Vec { origin, end }
    }

    pub fn parameterized_form(origin: Vec3, direction: Vec3) -> Self {
        Line3::Parameterized { origin, direction }
    }

    pub fn into_parameterized(self) -> Self {
        match self {
            Self::Vec { origin, end } => {
                let direction = end - origin;

                Self::Parameterized { origin, direction }
            }
            Self::Parameterized { origin, direction } => {
                Self::Parameterized { origin, direction }
            }
        }
    }

    pub fn origin(&self) -> Vec3 {
        match self {
            Self::Vec { origin, .. }
            | Self::Parameterized { origin, .. } => {
                *origin
            }
        }
    }

    pub fn end(&self) -> Vec3 {
        match self {
            Self::Vec { end, .. } => {
                *end
            }
            Self::Parameterized { origin, direction } => {
                *origin + *direction
            }
        }
    }

    pub fn direction(&self) -> Vec3 {
        match self {
            Self::Parameterized { direction, .. } => {
                *direction
            },
            _ => self.into_parameterized().direction()
        }
    }
}

impl Triangle {
    pub fn lines(&self) -> [Line3; 3] {
        [
            Line3::vertex_form(self.vertices[0], self.vertices[1]),
            Line3::vertex_form(self.vertices[1], self.vertices[2]),
            Line3::vertex_form(self.vertices[2], self.vertices[0])
        ]
    }
}

impl From<[[f32; 3]; 4]> for Triangle {
    fn from(data: [[f32; 3]; 4]) -> Self {
        Triangle {
            normal: data[3].into(),
            vertices: [
                data[0].into(),
                data[1].into(),
                data[2].into()
            ]
        }
    }
}
