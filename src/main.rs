#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "generate_bindings")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs")); // use build.rs generated
#[cfg(not(feature = "generate_bindings"))]
include!("./bindings.rs"); // use manually generated

use std::mem;
mod cpu;

fn main() {
    let mut memory: glibtop_mem;
    unsafe {
        memory = mem::zeroed();
        glibtop_init();
        glibtop_get_mem(&mut memory);
        glibtop_close();
    }
    println!("{:#?}", memory);
}
