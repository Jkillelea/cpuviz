#![allow(warnings)]
extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
  // jacob@murderbot:~$ pkg-config --libs libgtop-2.0
  // -lgtop-2.0 -lglib-2.0
  println!("cargo:rustc-link-lib=gtop-2.0");
  println!("cargo:rustc-link-lib=glib-2.0");

  #[cfg(not(feature = "generate_bindings"))]
  return; // generate the bindings later

  // --- generating new bindings, takes 20+ seconds ---

  // // jacob@murderbot:~$ pkg-config --cflags libgtop-2.0
  // // -I/usr/include/libgtop-2.0 -I/usr/include/glib-2.0
  // // -I/usr/lib/x86_64-linux-gnu/glib-2.0/include
  let bindings = bindgen::Builder::default()
        .trust_clang_mangling(false) // mangling breaks linking
        .clang_arg("-I/usr/include/libgtop-2.0") // include path
        .clang_arg("-I/usr/include/glib-2.0")
        .clang_arg("-I/usr/lib/x86_64-linux-gnu/glib-2.0/include")
        .header("wrapper.h") // add our headers here
        .generate()
        .expect("Couldn't generate headers");

  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

  bindings.write_to_file(out_path.join("bindings.rs"))
          .expect("couldn't write to file");
}
