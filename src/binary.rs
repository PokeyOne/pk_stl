use std::slice::Iter;

use crate::error::{Error, Result};
use crate::StlModel;
use crate::geometry::{Vec3, Triangle};

pub fn parse_binary_stl(bytes: &[u8]) -> Result<StlModel> {
    let mut data = bytes.into_iter();

    let header: Vec<u8> = data.by_ref().take(80).map(|val| { *val }).collect();
    let header: String = String::from_utf8_lossy(&header).trim_end_matches("\0").to_string();

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