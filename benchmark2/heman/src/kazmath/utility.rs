use ::libc;
#[no_mangle]
pub unsafe extern "C" fn kmSQR(mut s: libc::c_float) -> libc::c_float {
    return s * s;
}
#[no_mangle]
pub unsafe extern "C" fn kmDegreesToRadians(
    mut degrees: libc::c_float,
) -> libc::c_float {
    return degrees * (3.14159265358979323846f32 / 180.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn kmRadiansToDegrees(
    mut radians: libc::c_float,
) -> libc::c_float {
    return (radians as libc::c_double
        * (180.0f64 / 3.14159265358979323846f32 as libc::c_double)) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn kmMin(
    mut lhs: libc::c_float,
    mut rhs: libc::c_float,
) -> libc::c_float {
    return if lhs < rhs { lhs } else { rhs };
}
#[no_mangle]
pub unsafe extern "C" fn kmMax(
    mut lhs: libc::c_float,
    mut rhs: libc::c_float,
) -> libc::c_float {
    return if lhs > rhs { lhs } else { rhs };
}
#[no_mangle]
pub unsafe extern "C" fn kmAlmostEqual(
    mut lhs: libc::c_float,
    mut rhs: libc::c_float,
) -> libc::c_uchar {
    return (lhs as libc::c_double + 0.0001f64 > rhs as libc::c_double
        && lhs as libc::c_double - 0.0001f64 < rhs as libc::c_double) as libc::c_int
        as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn kmClamp(
    mut x: libc::c_float,
    mut min: libc::c_float,
    mut max: libc::c_float,
) -> libc::c_float {
    return if x < min { min } else if x > max { max } else { x };
}
#[no_mangle]
pub unsafe extern "C" fn kmLerp(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut t: libc::c_float,
) -> libc::c_float {
    return x + t * (y - x);
}
