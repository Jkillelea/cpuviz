mod gtop {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(warnings)]

    #[cfg(feature = "generate_bindings")]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs")); // use build.rs generated

    #[cfg(not(feature = "generate_bindings"))]
    include!("./bindings.rs"); // use manually generated
}

use std::thread;
use std::time::Duration;
mod cpu;
pub use cpu::Cpu;

fn main() {

    let mut cpu = cpu::Cpu::new();
    for _ in 0 .. 10 {
        thread::sleep(Duration::from_millis(1000));
        let (j, _) = cpu.measure();
        println!("{:?}", j);
    }
}
