use super::*;

const TOLERANCE: f32 = 1e-6;
const ONE_DEGREE: f32 = std::f32::consts::PI / 180.0;

const X: Vec3 = Vec3 {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};
const Y: Vec3 = Vec3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};
const Z: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};
const NEG_Z: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: -1.0,
};
const ZERO: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

/// Counter-clockwise triangle in the XY plane when viewed from +Z.
const CCW_XY: [[f32; 3]; 3] = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];

fn assert_approx_eq(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() <= TOLERANCE,
        "expected {expected}, got {actual}"
    );
}

fn assert_vec_approx_eq(actual: Vec3, expected: Vec3) {
    assert!(
        (actual.x - expected.x).abs() <= TOLERANCE
            && (actual.y - expected.y).abs() <= TOLERANCE
            && (actual.z - expected.z).abs() <= TOLERANCE,
        "expected {expected:?}, got {actual:?}"
    );
}

/// Edge vectors (v1 - v0, v2 - v0) of a triangle.
fn edges(v0: [f32; 3], v1: [f32; 3], v2: [f32; 3]) -> (Vec3, Vec3) {
    (
        Vec3::new([v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]]),
        Vec3::new([v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]]),
    )
}

/// Build a triangle from a normal and plain vertex arrays.
fn triangle(normal: impl Into<Vec3>, vertices: [[f32; 3]; 3]) -> Triangle {
    Triangle::new(normal.into(), vertices.map(Vec3::from))
}

/// Unit vector tilted away from +Z by `angle_rad` (rotation about X).
fn tilted_from_z(angle_rad: f32) -> [f32; 3] {
    [0.0, angle_rad.sin(), angle_rad.cos()]
}

// ---------------------------------------------------------------------------
// dot
// ---------------------------------------------------------------------------

#[test]
fn dot_known_value() {
    let a = Vec3::new([1.0, 2.0, 3.0]);
    let b = Vec3::new([4.0, 5.0, 6.0]);
    assert_eq!(a.dot(b), 32.0);
}

#[test]
fn dot_with_negative_components() {
    let a = Vec3::new([1.0, -2.0, 3.0]);
    let b = Vec3::new([-4.0, 5.0, 6.0]);
    assert_eq!(a.dot(b), 4.0);
}

#[test]
fn dot_is_commutative() {
    let a = Vec3::new([1.5, -2.25, 3.0]);
    let b = Vec3::new([-0.5, 4.0, 2.75]);
    assert_eq!(a.dot(b), b.dot(a));
}

#[test]
fn dot_of_orthogonal_vectors_is_zero() {
    assert_eq!(X.dot(Y), 0.0);
    assert_eq!(Y.dot(Z), 0.0);
    assert_eq!(Z.dot(X), 0.0);
}

#[test]
fn dot_with_zero_is_zero() {
    assert_eq!(Vec3::new([1.0, 2.0, 3.0]).dot(ZERO), 0.0);
}

#[test]
fn dot_with_self_is_length_squared() {
    let a = Vec3::new([2.0, 3.0, 6.0]);
    assert_eq!(a.dot(a), 49.0);
}

// ---------------------------------------------------------------------------
// length
// ---------------------------------------------------------------------------

#[test]
fn length_of_zero_vector_is_zero() {
    assert_eq!(ZERO.length(), 0.0);
}

#[test]
fn length_of_unit_axes_is_one() {
    assert_eq!(X.length(), 1.0);
    assert_eq!(Y.length(), 1.0);
    assert_eq!(Z.length(), 1.0);
}

#[test]
fn length_known_values() {
    assert_eq!(Vec3::new([3.0, 4.0, 0.0]).length(), 5.0);
    assert_eq!(Vec3::new([2.0, 3.0, 6.0]).length(), 7.0);
}

#[test]
fn length_ignores_sign() {
    assert_eq!(Vec3::new([-2.0, -3.0, -6.0]).length(), 7.0);
    assert_eq!(Vec3::new([2.0, -3.0, 6.0]).length(), 7.0);
}

// ---------------------------------------------------------------------------
// cross
// ---------------------------------------------------------------------------

#[test]
fn cross_of_unit_axes_follows_right_hand_rule() {
    assert_eq!(X.cross(Y), Z);
    assert_eq!(Y.cross(Z), X);
    assert_eq!(Z.cross(X), Y);
}

#[test]
fn cross_known_value() {
    let a = Vec3::new([1.0, 2.0, 3.0]);
    let b = Vec3::new([4.0, 5.0, 6.0]);
    assert_eq!(a.cross(b), Vec3::new([-3.0, 6.0, -3.0]));
}

#[test]
fn cross_is_anticommutative() {
    let a = Vec3::new([1.5, -2.0, 3.0]);
    let b = Vec3::new([-4.0, 0.5, 2.0]);
    let ba = b.cross(a);
    assert_eq!(a.cross(b), Vec3::new([-ba.x, -ba.y, -ba.z]));
}

#[test]
fn cross_with_self_is_zero() {
    let a = Vec3::new([1.0, 2.0, 3.0]);
    assert_eq!(a.cross(a), ZERO);
}

#[test]
fn cross_of_parallel_vectors_is_zero() {
    let a = Vec3::new([1.0, 2.0, 3.0]);
    let b = Vec3::new([-2.0, -4.0, -6.0]);
    assert_eq!(a.cross(b), ZERO);
}

#[test]
fn cross_is_orthogonal_to_both_inputs() {
    let a = Vec3::new([1.0, 2.0, 3.0]);
    let b = Vec3::new([-4.0, 5.0, 0.5]);
    let c = a.cross(b);
    assert_approx_eq(c.dot(a), 0.0);
    assert_approx_eq(c.dot(b), 0.0);
}

#[test]
fn cross_length_is_twice_triangle_area() {
    // Right triangle with legs 3 and 4 has area 6.
    let (a, b) = edges([1.0, 1.0, 1.0], [4.0, 1.0, 1.0], [1.0, 5.0, 1.0]);
    assert_eq!(a.cross(b).length(), 12.0);
}

// ---------------------------------------------------------------------------
// normalized
// ---------------------------------------------------------------------------

#[test]
fn normalized_unit_vector_is_unchanged() {
    assert_eq!(X.normalized(), Some(X));
    assert_eq!(Y.normalized(), Some(Y));
    assert_eq!(Z.normalized(), Some(Z));
}

#[test]
fn normalized_known_value() {
    let n = Vec3::new([3.0, 4.0, 0.0]).normalized().unwrap();
    assert_vec_approx_eq(n, Vec3::new([0.6, 0.8, 0.0]));
}

#[test]
fn normalized_has_unit_length() {
    let vectors = [
        Vec3::new([1.0, 2.0, 3.0]),
        Vec3::new([-7.5, 0.25, 12.0]),
        Vec3::new([1000.0, -2000.0, 3000.0]),
        Vec3::new([0.001, 0.002, -0.003]),
    ];
    for v in vectors {
        let n = v.normalized().unwrap();
        assert_approx_eq(n.length(), 1.0);
    }
}

#[test]
fn normalized_preserves_direction() {
    let v = Vec3::new([-2.0, 3.0, 6.0]);
    let n = v.normalized().unwrap();
    assert_vec_approx_eq(n, Vec3::new([-2.0 / 7.0, 3.0 / 7.0, 6.0 / 7.0]));
    assert!(n.dot(v) > 0.0);
}

#[test]
fn normalized_small_but_valid_vector_is_some() {
    // Length is far below f32::EPSILON but still safe to divide by.
    assert_eq!(Vec3::new([1e-10, 0.0, 0.0]).normalized(), Some(X));
}

#[test]
fn normalized_sub_normal_vector_is_some() {
    // f32::MIN_POSITIVE is the smallest *normal* value, so halve it to get a
    // subnormal component.
    assert_eq!(
        Vec3::new([f32::MIN_POSITIVE / 2.0, 0.0, 0.0]).normalized(),
        Some(X)
    );
}

#[test]
fn normalized_zero_vector_is_none() {
    assert_eq!(ZERO.normalized(), None);
}

#[test]
fn normalized_underflowing_vector_is_some() {
    // Squaring 1e-30 underflows f32 to zero using naive arithmetic, but should
    // be able to handle this.
    let n = Vec3::new([1e-30, 1e-30, 1e-30]).normalized().unwrap();
    let c = 1.0 / 3.0_f32.sqrt();
    assert_vec_approx_eq(n, Vec3::new([c, c, c]));
    assert_approx_eq(n.length(), 1.0);
}

#[test]
fn normalized_nan_vector_is_none() {
    assert_eq!(Vec3::new([f32::NAN, 1.0, 1.0]).normalized(), None);
}

#[test]
fn normalized_infinite_vector_is_none() {
    assert_eq!(Vec3::new([f32::INFINITY, 1.0, 1.0]).normalized(), None);
}

#[test]
fn normalized_large_vector_does_not_overflow() {
    // Squaring any of these components would overflow f32 to infinity.
    assert_eq!(Vec3::new([1e20, 0.0, 0.0]).normalized(), Some(X));

    let n = Vec3::new([3e30, -4e30, 0.0]).normalized().unwrap();
    assert_vec_approx_eq(n, Vec3::new([0.6, -0.8, 0.0]));

    let n = Vec3::new([f32::MAX, f32::MAX, f32::MAX])
        .normalized()
        .unwrap();
    let c = 1.0 / 3.0_f32.sqrt();
    assert_vec_approx_eq(n, Vec3::new([c, c, c]));
    assert_approx_eq(n.length(), 1.0);
}

// ---------------------------------------------------------------------------
// Triangle normals (cross + normalized together)
// ---------------------------------------------------------------------------

#[test]
fn counter_clockwise_triangle_normal_points_toward_viewer() {
    let (a, b) = edges([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    assert_eq!(a.cross(b).normalized(), Some(Z));
}

#[test]
fn clockwise_triangle_normal_points_away_from_viewer() {
    let (a, b) = edges([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]);
    assert_eq!(a.cross(b).normalized(), Some(Vec3::new([0.0, 0.0, -1.0])));
}

#[test]
fn degenerate_triangle_has_no_normal() {
    let (a, b) = edges([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [2.0, 2.0, 2.0]);
    assert_eq!(a.cross(b).normalized(), None);
}

// ---------------------------------------------------------------------------
// Triangle::new
// ---------------------------------------------------------------------------

#[test]
fn triangle_new_stores_fields_unchanged() {
    // Deliberately wrong normal: construction must not validate or fix it.
    let t = triangle([5.0, 5.0, 5.0], CCW_XY);
    assert_eq!(t.normal, Vec3::new([5.0, 5.0, 5.0]));
    assert_eq!(t.vertices, CCW_XY.map(Vec3::from));
}

// ---------------------------------------------------------------------------
// Triangle::calculate_normal
// ---------------------------------------------------------------------------

#[test]
fn calculate_normal_counter_clockwise_points_toward_viewer() {
    assert_eq!(triangle(ZERO, CCW_XY).calculate_normal(), Some(Z));
}

#[test]
fn calculate_normal_clockwise_points_away_from_viewer() {
    let [a, b, c] = CCW_XY;
    assert_eq!(triangle(ZERO, [a, c, b]).calculate_normal(), Some(NEG_Z));
}

#[test]
fn calculate_normal_is_unaffected_by_cyclic_vertex_order() {
    let [a, b, c] = CCW_XY;
    for vertices in [[a, b, c], [b, c, a], [c, a, b]] {
        assert_vec_approx_eq(triangle(ZERO, vertices).calculate_normal().unwrap(), Z);
    }
}

#[test]
fn calculate_normal_oblique_triangle() {
    let t = triangle(ZERO, [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    let c = 1.0 / 3.0_f32.sqrt();
    assert_vec_approx_eq(t.calculate_normal().unwrap(), Vec3::new([c, c, c]));
}

#[test]
fn calculate_normal_is_translation_invariant() {
    let offset = [120.5, -40.25, 999.0];
    let moved = CCW_XY.map(|v| [v[0] + offset[0], v[1] + offset[1], v[2] + offset[2]]);
    assert_vec_approx_eq(triangle(ZERO, moved).calculate_normal().unwrap(), Z);
}

#[test]
fn calculate_normal_tiny_triangle() {
    // Sub-millimetre triangle in a model stored in metres.
    let tiny = CCW_XY.map(|v| v.map(|c| c * 1e-5));
    assert_eq!(triangle(ZERO, tiny).calculate_normal(), Some(Z));
}

#[test]
fn calculate_normal_ignores_stored_normal() {
    let t = triangle([f32::NAN, 7.0, -3.0], CCW_XY);
    assert_eq!(t.calculate_normal(), Some(Z));
}

#[test]
fn calculate_normal_collinear_is_none() {
    let t = triangle(ZERO, [[0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [2.0, 2.0, 2.0]]);
    assert_eq!(t.calculate_normal(), None);
}

#[test]
fn calculate_normal_duplicate_vertices_is_none() {
    let t = triangle(ZERO, [[1.0, 2.0, 3.0], [1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
    assert_eq!(t.calculate_normal(), None);

    let t = triangle(ZERO, [[1.0, 2.0, 3.0]; 3]);
    assert_eq!(t.calculate_normal(), None);
}

// ---------------------------------------------------------------------------
// Triangle::recalculate_normal
// ---------------------------------------------------------------------------

#[test]
fn recalculate_normal_overwrites_wrong_normal() {
    let mut t = triangle(X, CCW_XY);
    let returned = t.recalculate_normal();
    assert_eq!(returned, Z);
    assert_eq!(t.normal, Z);
}

#[test]
fn recalculate_normal_leaves_vertices_unchanged() {
    let mut t = triangle(NEG_Z, CCW_XY);
    let before = t.vertices;
    t.recalculate_normal();
    assert_eq!(t.vertices, before);
}

#[test]
fn recalculate_normal_degenerate_writes_zero() {
    let mut t = triangle(Z, [[1.0, 1.0, 1.0]; 3]);
    assert_eq!(t.recalculate_normal(), ZERO);
    assert_eq!(t.normal, ZERO);
}

#[test]
fn recalculate_normal_result_verifies_ok() {
    let mut t = triangle(
        [0.3, -0.2, 0.9],
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    );
    t.recalculate_normal();
    assert_eq!(t.verify_normal(ONE_DEGREE), NormalVerificationResult::Ok);
}

// ---------------------------------------------------------------------------
// Triangle::verify_normal
// ---------------------------------------------------------------------------

#[test]
fn verify_normal_exact_match_is_ok() {
    let t = triangle(Z, CCW_XY);
    assert_eq!(t.verify_normal(ONE_DEGREE), NormalVerificationResult::Ok);
}

#[test]
fn verify_normal_within_tolerance_is_ok() {
    let t = triangle(tilted_from_z(0.5 * ONE_DEGREE), CCW_XY);
    assert_eq!(t.verify_normal(ONE_DEGREE), NormalVerificationResult::Ok);
}

#[test]
fn verify_normal_non_unit_stored_normal_is_ok() {
    // Direction is right, only the length is wrong.
    let t = triangle([0.0, 0.0, 42.0], CCW_XY);
    assert_eq!(t.verify_normal(ONE_DEGREE), NormalVerificationResult::Ok);
}

#[test]
fn verify_normal_beyond_tolerance_is_mismatch_with_angle() {
    let angle = 5.0 * ONE_DEGREE;
    let t = triangle(tilted_from_z(angle), CCW_XY);
    match t.verify_normal(ONE_DEGREE) {
        NormalVerificationResult::Mismatch { angle_rad } => {
            assert!(
                (angle_rad - angle).abs() < 1e-3,
                "expected ~{angle}, got {angle_rad}"
            );
        }
        other => panic!("expected Mismatch, got {other:?}"),
    }
}

#[test]
fn verify_normal_perpendicular_is_mismatch() {
    let t = triangle(X, CCW_XY);
    match t.verify_normal(ONE_DEGREE) {
        NormalVerificationResult::Mismatch { angle_rad } => {
            assert!((angle_rad - std::f32::consts::FRAC_PI_2).abs() < 1e-5);
        }
        other => panic!("expected Mismatch, got {other:?}"),
    }
}

#[test]
fn verify_normal_opposite_is_flipped() {
    let t = triangle(NEG_Z, CCW_XY);
    assert_eq!(
        t.verify_normal(ONE_DEGREE),
        NormalVerificationResult::Flipped
    );
}

#[test]
fn verify_normal_nearly_opposite_is_flipped() {
    let t = triangle(tilted_from_z(179.5 * ONE_DEGREE), CCW_XY);
    assert_eq!(
        t.verify_normal(ONE_DEGREE),
        NormalVerificationResult::Flipped
    );
}

#[test]
fn verify_normal_clockwise_winding_is_flipped() {
    let [a, b, c] = CCW_XY;
    let t = triangle(Z, [a, c, b]);
    assert_eq!(
        t.verify_normal(ONE_DEGREE),
        NormalVerificationResult::Flipped
    );
}

#[test]
fn verify_normal_between_mismatch_and_flipped_is_mismatch() {
    let t = triangle(tilted_from_z(170.0 * ONE_DEGREE), CCW_XY);
    assert!(matches!(
        t.verify_normal(ONE_DEGREE),
        NormalVerificationResult::Mismatch { .. }
    ));
}

#[test]
fn verify_normal_zero_is_missing() {
    let t = triangle(ZERO, CCW_XY);
    assert_eq!(
        t.verify_normal(ONE_DEGREE),
        NormalVerificationResult::Missing
    );
}

#[test]
fn verify_normal_negative_zero_is_missing() {
    let t = triangle([-0.0, 0.0, -0.0], CCW_XY);
    assert_eq!(
        t.verify_normal(ONE_DEGREE),
        NormalVerificationResult::Missing
    );
}

#[test]
fn verify_normal_nan_is_invalid_normal() {
    let t = triangle([f32::NAN, 0.0, 1.0], CCW_XY);
    assert_eq!(
        t.verify_normal(ONE_DEGREE),
        NormalVerificationResult::InvalidNormal
    );
}

#[test]
fn verify_normal_infinite_is_invalid_normal() {
    let t = triangle([0.0, 0.0, f32::INFINITY], CCW_XY);
    assert_eq!(
        t.verify_normal(ONE_DEGREE),
        NormalVerificationResult::InvalidNormal
    );
}

#[test]
fn verify_normal_degenerate_triangle_is_invalid_triangle() {
    let t = triangle(Z, [[1.0, 1.0, 1.0]; 3]);
    assert_eq!(
        t.verify_normal(ONE_DEGREE),
        NormalVerificationResult::InvalidTriangle
    );
}

#[test]
fn verify_normal_missing_takes_precedence_over_invalid_triangle() {
    let t = triangle(ZERO, [[1.0, 1.0, 1.0]; 3]);
    assert_eq!(
        t.verify_normal(ONE_DEGREE),
        NormalVerificationResult::Missing
    );
}

#[test]
fn verify_normal_invalid_normal_takes_precedence_over_invalid_triangle() {
    let t = triangle([f32::NAN; 3], [[1.0, 1.0, 1.0]; 3]);
    assert_eq!(
        t.verify_normal(ONE_DEGREE),
        NormalVerificationResult::InvalidNormal
    );
}
