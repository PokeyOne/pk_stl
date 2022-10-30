use pk_stl::parse_stl;

#[test]
fn test_binary_cube() {
    let content = include_bytes!("test_cube.stl");
    let model = parse_stl(content).unwrap();

    assert_eq!(model.header, "OpenSCAD Model\n");
    assert_eq!(model.triangles.len(), 12);

    assert_eq!(model.dimension_range(), Some(((0.0, 10.0), (0.0, 10.0), (0.0, 10.0))));
}