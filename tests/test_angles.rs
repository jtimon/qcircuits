
/// Basic testing for Angle class

use qcircuits::angle::{
    Angle,
    MAX_ANGLE,
};

fn test_angle_addition_pair(expected: u16, a: u16, b: u16) {

    assert_eq!(Angle::new(expected), Angle::new(a) + b);
    assert_eq!(Angle::new(expected), Angle::new(b) + a);
    assert_eq!(Angle::new(expected), Angle::new(a) + Angle::new(b));
}

fn test_angle_substraction_pair(expected: u16, a: u16, b: u16) {

    assert_eq!(Angle::new(expected), Angle::new(a) - b);
    assert_eq!(Angle::new(expected), Angle::new(a) - Angle::new(b));
}

#[test]
fn test_angle_addition() {
    test_angle_addition_pair(0, 0, MAX_ANGLE);
    test_angle_addition_pair(50, 30, 20);
    test_angle_addition_pair(270, 90, 180);
    test_angle_addition_pair(280, 90, 190);
    test_angle_addition_pair(0, 180, 180);
    test_angle_addition_pair(180, 270, 270);

    let mut a = Angle::new(10);
    a = a + Angle::new(20);
    assert_eq!(Angle::new(30), a);
}

#[test]
fn test_angle_substraction() {
    test_angle_substraction_pair(0, 0, MAX_ANGLE);
    test_angle_substraction_pair(10, 30, 20);
    test_angle_substraction_pair(270, 90, 180);
    test_angle_substraction_pair(260, 90, 190);
    test_angle_substraction_pair(0, 180, 180);
    test_angle_substraction_pair(0, 270, 270);

    let mut a = Angle::new(10);
    a = a - Angle::new(20);
    assert_eq!(Angle::new(350), a);
}
