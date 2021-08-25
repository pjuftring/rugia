#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    unused_variables,
    dead_code,
    clippy::all
)]

pub mod gl {
    include!("generated/gl.rs");
}
pub mod glfw {
    include!("generated/glfw.rs");
}
pub mod webp {
    include!("generated/webp.rs");
}
