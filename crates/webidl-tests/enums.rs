use js_sys::Object;
use wasm_bindgen_test::*;

include!(concat!(env!("OUT_DIR"), "/enums.rs"));

#[wasm_bindgen_test]
fn top_level_enum() {
    let circle = Shape::new(ShapeType::Circle).unwrap();
    let square = Shape::new(ShapeType::Square).unwrap();
    assert!(circle.is_circle());
    assert!(!circle.is_square());
    assert!(square.is_square());
    assert!(!square.is_circle());
}

#[wasm_bindgen_test]
fn valid_enum_return() {
    let circle = Shape::new(ShapeType::Circle).unwrap();
    let square = Shape::new(ShapeType::Square).unwrap();
    assert!(circle.is_circle());
    assert!(!circle.is_square());
    assert_eq!(circle.get_shape(), ShapeType::Circle);
    assert!(square.is_square());
    assert!(!square.is_circle());
    assert_eq!(square.get_shape(), ShapeType::Square);
}

#[wasm_bindgen_test]
fn invalid_enum_return() {
    let actually_a_triangle = Shape::triangle();
    assert!(!actually_a_triangle.is_circle());
    assert!(!actually_a_triangle.is_square());
    match actually_a_triangle.get_shape() {
        ShapeType::Circle | ShapeType::Square => assert!(false),
        _ => {} // Success
    };
}
