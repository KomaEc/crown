#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(register_tool)]
#![register_tool(c2rust)]
#![feature(strict_provenance)]
#![feature(core_intrinsics)]
#![feature(raw_ref_op)]

extern crate libc;
pub mod src {
pub mod kazmath {
pub mod aabb2;
pub mod aabb3;
pub mod mat3;
pub mod mat4;
pub mod plane;
pub mod quaternion;
pub mod ray2;
pub mod ray3;
pub mod utility;
pub mod vec2;
pub mod vec3;
pub mod vec4;
} // mod kazmath
pub mod src {
pub mod color;
pub mod distance;
pub mod draw;
pub mod export;
pub mod gaussian;
pub mod generate;
pub mod image;
pub mod import;
pub mod lighting;
pub mod noise;
pub mod ops;
pub mod points;
} // mod src
} // mod src
