use std::ops::{Add, Sub, Mul};

/// A 3D vector.
///
/// This structure is used to provide extra mathematical operations on top of
/// the standard 3D array or vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    /// The x coordinate of the vector.
    pub x: f32,
    /// The y coordinate of the vector.
    pub y: f32,
    /// The z coordinate of the vector.
    pub z: f32
}

/// A single triangle in a model.
///
/// This is the base 3D shape of an STL model. It is composed of a normal vector
/// and three vertices.
///
/// The normal vector is not verified to be correct, and a model file may give
/// incorrect values. Currently there is no way to verify or calculate the normals
/// using this library, however v0.4 will include methods
/// [`verify_normal`] and [`calculate_normal`]. These methods will be able to
/// verify and calculate normals.
///
/// The triangle can be initialized through an array of four vertices, where the
/// first 3 are the vertices and the last is the normal vector. This is the
/// same order that is used in the binary STL format.
///
/// # Examples
///
/// ```
/// use pk_stl::geometry::Triangle;
///
/// let data = [
///     [0.0, 0.0, 0.0],
///     [1.0, 0.0, 0.0],
///     [0.0, 1.0, 0.0],
///     [0.0, 0.0, 1.0]
/// ];
/// let triangle = Triangle::from(data);
///
/// assert_eq!(triangle.normal, [0.0, 0.0, 1.0].into());
/// assert_eq!(triangle.vertices, [
///    [0.0, 0.0, 0.0].into(),
///    [1.0, 0.0, 0.0].into(),
///    [0.0, 1.0, 0.0].into()
/// ]);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    /// The normal value of the triangle. Not verified to be correct.
    pub normal: Vec3,
    /// The three vertices of the triangle.
    pub vertices: [Vec3; 3]
}

impl Vec3 {
    /// Create a new Vec3 from an array of three values.
    pub fn new(data: [f32; 3]) -> Vec3 {
        Vec3 {
            x: data[0],
            y: data[1],
            z: data[2]
        }
    }

    /// Create an array of three values from a Vec3.
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
