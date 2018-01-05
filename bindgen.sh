bindgen wrapper.h --distrust-clang-mangling -- `pkg-config --cflags libgtop-2.0` > src/bindings.rs
