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

use std::slice::Iter;
use std::fmt::Write;

pub mod geometry;
pub mod error;

#[cfg(test)]
mod tests;

use geometry::{Vec3, Triangle};
use error::{Error, Result};

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
        parse_ascii_stl(bytes)
    } else {
        parse_binary_stl(bytes)
    }
}

fn parse_binary_stl(bytes: &[u8]) -> Result<StlModel> {
    let mut data = bytes.into_iter();

    let header: Vec<u8> = data.by_ref().take(80).map(|val| { *val }).collect();
    let header: String = String::from_utf8_lossy(&header).trim_end_matches("\0").to_string();

    println!("utf8 of header: {}", header.escape_debug());

    let triangle_count = {
        let mut raw = [0; 4];

        for i in 0..4 {
            raw[i] = match data.next() {
                Some(val) => *val,
                None => return Err(Error::binary("Invalid trianlge count byte sequence"))
            }
        }

        u32::from_le_bytes(raw)
    };

    println!("Triangle count: {triangle_count}");

    let mut triangles: Vec<Triangle> = Vec::with_capacity(triangle_count as usize);

    for _ in 0..(triangle_count as usize) {
        let normal = read_f32_triplet(&mut data)?;
        let vert_a = read_f32_triplet(&mut data)?;
        let vert_b = read_f32_triplet(&mut data)?;
        let vert_c = read_f32_triplet(&mut data)?;

        // For now we just ignore the attribute byte count
        // TODO: Possibly support attributes, but not priority.
        let _ = data.next();
        let _ = data.next();

        triangles.push(Triangle {
            normal: Vec3::new(normal),
            vertices: [
                Vec3::new(vert_a),
                Vec3::new(vert_b),
                Vec3::new(vert_c)
            ]
        })
    }

    Ok(StlModel { header, triangles })
}

fn read_f32_triplet<'a>(data: &mut Iter<'a, u8>) -> Result<[f32; 3]> {
    Ok([
        read_f32(data)?,
        read_f32(data)?,
        read_f32(data)?
    ])
}

fn read_f32<'a>(data: &mut Iter<'a, u8>) -> Result<f32> {
    let mut raw = [0; 4];
    for item in &mut raw {
        *item = match data.next() {
            Some(val) => *val,
            None => return Err(Error::binary("Invalid trianlge count byte sequence"))
        };
    }

    Ok(f32::from_le_bytes(raw))
}

fn parse_ascii_stl(_bytes: &[u8]) -> Result<StlModel> {
    Err(Error::ascii("Ascii files not implemented yet"))
}
