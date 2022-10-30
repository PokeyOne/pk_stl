use crate::geometry::Vec3;

use super::*;

#[test]
fn test_dimension_range() {
    let model = StlModel {
        header: String::new(),
        triangles: vec![
            Triangle {
                normal: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
                vertices: [
                    Vec3 { x: 0.0, y: 0.0, z: 5.0 },
                    Vec3 { x: 1.0, y: 0.0, z: 0.0 },
                    Vec3 { x: 0.0, y: 1.0, z: 0.0 },
                ]
            },
            Triangle {
                normal: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
                vertices: [
                    Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                    Vec3 { x: 1.0, y: 0.0, z: -1.0 },
                    Vec3 { x: 0.0, y: 1.0, z: 0.0 },
                ]
            },
        ]
    };

    assert_eq!(
        model.dimension_range(),
        Some(((0.0, 1.0), (0.0, 1.0), (-1.0, 5.0)))
    );
}

#[test]
fn test_dimension_range_for_empty_model() {
    let model = StlModel {
        header: String::new(),
        triangles: vec![]
    };

    assert_eq!(model.dimension_range(), None);
}

#[test]
fn test_as_binary() {
    let model = StlModel {
        header: String::new(),
        triangles: vec![
            Triangle {
                normal: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
                vertices: [
                    Vec3 { x: 0.0, y: 0.0, z: 5.0 },
                    Vec3 { x: 1.0, y: 0.0, z: 0.0 },
                    Vec3 { x: 0.0, y: 1.0, z: 0.0 },
                ]
            },
            Triangle {
                normal: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
                vertices: [
                    Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                    Vec3 { x: 1.0, y: 0.0, z: -1.0 },
                    Vec3 { x: 0.0, y: 1.0, z: 0.0 },
                ]
            },
        ]
    };

    let binary = model.as_binary();

    let reparsed_model = parse_stl(&binary).unwrap();

    assert_eq!(model, reparsed_model);
}