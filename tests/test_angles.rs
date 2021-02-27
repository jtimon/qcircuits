
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

#[test]
fn test_angle_between() {
    assert_eq!(true, Angle::new(0).between(0, 10)); // the initial angle is included (arbitrary)
    assert_eq!(false, Angle::new(10).between(0, 10)); // the final angle is excluded
    assert_eq!(true, Angle::new(30).between(20, 40));
    assert_eq!(false, Angle::new(20).between(30, 40));
    assert_eq!(false, Angle::new(0).between(90, 180));
    assert_eq!(true, Angle::new(20).between(40, 30));
    assert_eq!(true, Angle::new(50).between(40, 30));
    assert_eq!(false, Angle::new(35).between(40, 30));
    assert_eq!(true, Angle::new(40).between(40, 30));
    assert_eq!(false, Angle::new(30).between(40, 30));

    assert_eq!(true, Angle::new(90).between(0, 180));
    assert_eq!(true, Angle::new(180).between(90, 270));
    assert_eq!(true, Angle::new(180).between(80, 270));
    assert_eq!(true, Angle::new(270).between(225, 315));
    assert_eq!(true, Angle::new(280).between(270, 90));
    assert_eq!(true, Angle::new(0).between(270, 90));
    assert_eq!(true, Angle::new(320).between(315, 45));
    assert_eq!(true, Angle::new(10).between(315, 45));
    assert_eq!(true, Angle::new(315).between(315, 45));
    assert_eq!(false, Angle::new(45).between(315, 45));
}
