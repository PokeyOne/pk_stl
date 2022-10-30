//! STL file parsing and writing.
//!
//! This crate provides a simple interface for reading and writing STL files. It
//! is written entirely in Rust with no dependencies, and it can read and write
//! both ASCII and binary STL files.
//!
//! # Examples
//!
//! ```
//! use pk_stl::parse_stl;
//!
//! // Files may be loaded from bytes or from ascii.
//! let content = include_bytes!("../tests/test_cube.stl");
//! let model = parse_stl(content).unwrap();
//!
//! // Models can be converted between ascii and binary.
//! let ascii_content = model.as_ascii();
//!
//! // The header of this model is "OpenSCAD Model\n" because this file happens
//! // to be the output of OpenSCAD.
//! assert_eq!(ascii_content.lines().next(), Some("solid OpenSCAD Model"));
//! ```

use std::fmt::Write;

pub mod geometry;
pub mod error;

mod binary;
mod ascii;

#[cfg(test)]
mod tests;

use geometry::Triangle;
use error::Result;

/// The main structure of this crate. It represents a single STL model.
///
/// STL files are composed of a header and a list of triangles. This structure
/// represents both of those things.
#[derive(Debug, Clone)]
pub struct StlModel {
    /// The main header line of the STL file.
    ///
    /// Some STL files do use the header to convey information about the model,
    /// but this is not required. The header is not used by this crate.
    pub header: String,
    /// Each triangle in the model.
    pub triangles: Vec<Triangle>
}

impl StlModel {
    /// Convert the model to ASCII STL format.
    ///
    /// This will use the header of the model, trimmed with newlines removed.
    pub fn as_ascii(&self) -> String {
        let mut result = String::new();

        writeln!(result, "solid {}", self.header.trim().replace("\n", " ")).unwrap();

        for triangle in &self.triangles {
            writeln!(result, "facet normal {:e} {:e} {:e}", triangle.normal.x, triangle.normal.y, triangle.normal.z).unwrap();
            writeln!(result, "    outer loop").unwrap();
            for v in &triangle.vertices {
                writeln!(result, "        vertex {:e} {:e} {:e}", v.x, v.y, v.z).unwrap();
            }
            writeln!(result, "    endloop").unwrap();
            writeln!(result, "endfacet").unwrap();
        }

        result
    }

    /// Find the range of positions in the model.
    ///
    /// This will return and optional tuple of three ranges. The values is only
    /// `None` if there are no triangles in the model. Otherwise, the ranges are
    /// the minimum and maximum values for the x, y, and z coordinates of the
    /// model.
    ///
    /// For example, `Some(((0.0, 1.0), (2.0, 3.0), (4.0, 5.0)))` would mean
    /// that the model has the following minimum and maximum values:
    ///
    /// | Coordinate | Minimum | Maximum |
    /// |------------|---------|---------|
    /// | x          | 0.0     | 1.0     |
    /// | y          | 2.0     | 3.0     |
    /// | z          | 4.0     | 5.0     |
    ///
    /// This is useful for determining the size of the model.
    pub fn dimension_range(&self) -> Option<((f32, f32), (f32, f32), (f32, f32))> {
        let mut maybe_range: Option<((f32, f32), (f32, f32), (f32, f32))> = None;

        for triangle in self.triangles.iter() {
            for vertex in triangle.vertices.iter() {
                match maybe_range {
                    Some((x_range, y_range, z_range)) => {
                        maybe_range = Some((
                            (x_range.0.min(vertex.x), x_range.1.max(vertex.x)),
                            (y_range.0.min(vertex.y), y_range.1.max(vertex.y)),
                            (z_range.0.min(vertex.z), z_range.1.max(vertex.z))
                        ));
                    },
                    None => {
                        maybe_range = Some(((vertex.x, vertex.x), (vertex.y, vertex.y), (vertex.z, vertex.z)));
                    }
                }
            }
        }

        maybe_range
    }
}

/// Parse an STL file from bytes.
///
/// The bytes can be either ASCII or binary. Whether the file is ASCII or binary
/// will be determined by the first 6 bytes of the file. If the file starts
/// with "solid ", it will be parsed as ASCII. Otherwise, it will be parsed as
/// binary.
pub fn parse_stl(bytes: &[u8]) -> Result<StlModel> {
    if &bytes[0..6] == b"solid " {
        ascii::parse_ascii_stl(bytes)
    } else {
        binary::parse_binary_stl(bytes)
    }
}
