
#[no_mangle]
pub extern "C" fn kmSQR(mut s: std::os::raw::c_float) -> std::os::raw::c_float {
    return s * s;
}
#[no_mangle]
pub extern "C" fn kmDegreesToRadians(
    mut degrees: std::os::raw::c_float,
) -> std::os::raw::c_float {
    return degrees * (3.14159265358979323846f32 / 180.0f32);
}
#[no_mangle]
pub extern "C" fn kmRadiansToDegrees(
    mut radians: std::os::raw::c_float,
) -> std::os::raw::c_float {
    return (radians as std::os::raw::c_double
        * (180.0f64 / 3.14159265358979323846f32 as std::os::raw::c_double)) as std::os::raw::c_float;
}
#[no_mangle]
pub extern "C" fn kmMin(
    mut lhs: std::os::raw::c_float,
    mut rhs: std::os::raw::c_float,
) -> std::os::raw::c_float {
    return if lhs < rhs { lhs } else { rhs };
}
#[no_mangle]
pub extern "C" fn kmMax(
    mut lhs: std::os::raw::c_float,
    mut rhs: std::os::raw::c_float,
) -> std::os::raw::c_float {
    return if lhs > rhs { lhs } else { rhs };
}
#[no_mangle]
pub extern "C" fn kmAlmostEqual(
    mut lhs: std::os::raw::c_float,
    mut rhs: std::os::raw::c_float,
) -> std::os::raw::c_uchar {
    return (lhs as std::os::raw::c_double + 0.0001f64 > rhs as std::os::raw::c_double
        && lhs as std::os::raw::c_double - 0.0001f64 < rhs as std::os::raw::c_double) as std::os::raw::c_int
        as std::os::raw::c_uchar;
}
#[no_mangle]
pub extern "C" fn kmClamp(
    mut x: std::os::raw::c_float,
    mut min: std::os::raw::c_float,
    mut max: std::os::raw::c_float,
) -> std::os::raw::c_float {
    return if x < min { min } else if x > max { max } else { x };
}
#[no_mangle]
pub extern "C" fn kmLerp(
    mut x: std::os::raw::c_float,
    mut y: std::os::raw::c_float,
    mut t: std::os::raw::c_float,
) -> std::os::raw::c_float {
    return x + t * (y - x);
}
use crate::laertes_rt::*;
