#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//! Crate level documentation

/// All the generated rust code goes inside this module
/// We don't use macro here because IntelliJ Rust can not understand it
#[path = "../generated/rust/object_api.rs"]
pub mod object_api;
