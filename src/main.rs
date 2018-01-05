use std::thread;
use std::time::Duration;
use std::sync::Arc;

mod cpu;
mod glibtop_handle;
pub use cpu::Cpu;
pub use glibtop_handle::GLibTopHandle;

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

fn main() {
    let gtop_handle = Arc::new(GLibTopHandle::new()); // Atomic Reference Counted object to call glibtop_init and glibtop_close when we start and when we run out of objects

    let mut cpu = cpu::Cpu::with_handle(gtop_handle.clone());
    for _ in 0..100 {
        thread::sleep(Duration::from_secs(1));
        let (j, _) = cpu.measure();
        println!("{:?}", j);
    }
}
