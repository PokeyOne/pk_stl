use std::ops::{Add, Div, Mul, Sub};

#[cfg(test)]
mod tests;

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
    pub z: f32,
}

impl Vec3 {
    /// Create a new Vec3 from an array of three values.
    pub fn new(data: [f32; 3]) -> Vec3 {
        Vec3 {
            x: data[0],
            y: data[1],
            z: data[2],
        }
    }

    /// Create an array of three values from a Vec3.
    pub fn as_arr(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    /// Calculate the dot product between this vector and the given vector.
    pub fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Calculate the length of this vector.
    ///
    /// This is equivalent to the sqrt of the dot product with itself, or the
    /// square root of the sum of each axis squared.
    ///
    /// # Returns
    ///
    /// Returns the length of the vector. May be non-normal.
    pub fn length(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    /// Calculate the cross product between this vector and given vector.
    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    /// Normalize the vector to length 1.
    ///
    /// # Returns
    ///
    /// Return the calculated normalized vector, or None if the length is too
    /// small to normalize.
    pub fn normalized(&self) -> Option<Self> {
        // NOTE: Extra division to prevent cases like [1e20, 0, 0] from overflowing
        // in the squared step instead of going to [1, 0, 0]
        let max = self.x.abs().max(self.y.abs()).max(self.z.abs());
        if max == 0.0 || !max.is_finite() {
            return None;
        }

        let scaled = *self / max;
        let len = scaled.length();

        if !len.is_normal() {
            return None;
        }

        Some(scaled / len)
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
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

/// A single triangle in a model.
///
/// This is the base 3D shape of an STL model. It is composed of a normal vector
/// and three vertices.
///
/// The normal vector is not verified to be correct, and a model file may give
/// incorrect values. See [`Triangle::verify_normal`] and
/// [`Triangle::calculate_normal`] for verifying and calculating the normal, and
/// [`Triangle::recalculate_normal`] to simply replace the stored normal.
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
    pub vertices: [Vec3; 3],
}

impl Triangle {
    /// Creates a new triangle with the given normal and vertices.
    pub fn new(normal: Vec3, vertices: [Vec3; 3]) -> Self {
        Self { normal, vertices }
    }

    /// Calculate the normal vector for this triangle.
    ///
    /// This neither uses nor changes the internal `normal` field of this
    /// struct.
    ///
    /// See [`Triangle::recalculate_normal`] and [`Triangle::verify_normal`].
    ///
    /// # Returns
    ///
    /// Returns the normal vector of the triangle if it can be computed. None
    /// will be returned if the normal cannot be computed due to duplicate or
    /// colinear points.
    pub fn calculate_normal(&self) -> Option<Vec3> {
        let a = self.vertices[1] - self.vertices[0];
        let b = self.vertices[2] - self.vertices[0];

        a.cross(b).normalized()
    }

    /// Re-calculate the normal vector and update `normal`field.
    ///
    /// This uses the [`Triangle::calculate_normal`] method to calculate the
    /// normal, and writes [0, 0, 0] if the normal cannot be calculated.
    ///
    /// # Return
    ///
    /// Returns a copy of the new normal.
    pub fn recalculate_normal(&mut self) -> Vec3 {
        self.normal = self
            .calculate_normal()
            .unwrap_or(Vec3::new([0.0, 0.0, 0.0]));

        self.normal
    }

    /// Verify the stored normal against the computed normal vector.
    ///
    /// # Params
    ///
    /// * `tolerance_rad` - The maximum difference in radians between stored and
    ///    computed normal that is considered matching.
    ///
    /// The tolerance should be less than 90 deg, or pi/2
    pub fn verify_normal(&self, tolerance_rad: f32) -> NormalVerificationResult {
        if self.normal == Vec3::new([0.0, 0.0, 0.0]) {
            return NormalVerificationResult::Missing;
        }

        let Some(stored) = self.normal.normalized() else {
            return NormalVerificationResult::InvalidNormal;
        };

        let Some(expected) = self.calculate_normal() else {
            return NormalVerificationResult::InvalidTriangle;
        };

        let cos = stored.dot(expected).clamp(-1.0, 1.0);
        let min_cos = tolerance_rad.cos();

        if cos >= min_cos {
            NormalVerificationResult::Ok
        } else if cos <= -min_cos {
            NormalVerificationResult::Flipped
        } else {
            NormalVerificationResult::Mismatch {
                angle_rad: cos.acos(),
            }
        }
    }
}

impl From<[[f32; 3]; 4]> for Triangle {
    fn from(data: [[f32; 3]; 4]) -> Self {
        Triangle {
            normal: data[3].into(),
            vertices: [data[0].into(), data[1].into(), data[2].into()],
        }
    }
}

/// Result of running a normal verification.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NormalVerificationResult {
    /// Stored normal matches calculation.
    Ok,
    /// Normal is roughly the opposite of the calculated normal.
    Flipped,
    /// Stored normal does not match calculated.
    ///
    /// `angle_rad` is the angle between the stored normal and calculated
    /// normal.
    Mismatch { angle_rad: f32 },
    /// The stored normal is [0, 0, 0], which is considered not set.
    Missing,
    /// The stored normal was not a valid finite number.
    InvalidNormal,
    /// The normal of the triangle could not be calculated
    InvalidTriangle,
}

impl NormalVerificationResult {
    /// Return true if this value is [`Self::Ok`].
    pub fn is_ok(self) -> bool {
        self == Self::Ok
    }
}
