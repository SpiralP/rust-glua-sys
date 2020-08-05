# glua-sys

Rust bindings to LuaJIT function exports

## Example

Add this to `Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
classicube-sys = { git = "https://github.com/SpiralP/rust-glua-sys.git" }
```

`lib.rs`:

```rust
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

use glua_sys::*;
use std::{ffi::CString, os::raw::c_int};

#[no_mangle]
pub unsafe extern "C" fn gmod13_open(L: *mut lua_State) -> c_int {
    print(L, "Hello, world!");
    0
}

#[no_mangle]
pub unsafe extern "C" fn gmod13_close(_L: *mut lua_State) -> c_int {
    0
}

fn print<T: Into<Vec<u8>>>(L: *mut lua_State, s: T) {
    let c_string = CString::new(s).unwrap();
    let length = c_string.as_bytes().len();
    let c_print = CString::new("print").unwrap();

    unsafe {
        lua_getglobal!(L, c_print.as_ptr());
        lua_pushlstring(L, c_string.as_ptr(), length as _);
        // 1 arg, 1 result
        lua_call(L, 1, 1);
    }
}
```

## Linking problems

Windows will try to detect where your .dll is located and create a .lib to link to automagically, but other OS's don't do this.

On Linux you can tell ld to link specifically to `lua_shared.so` without the "lib" prefix using the `-l:lua_shared.so` flag. (or you can try `patchelf --add-needed lua_shared.so`)

Linux Srcds might not have `./garrysmod/bin` in the linker path to find `lua_shared_srv.so` so we use the `-Wl,-rpath,./garrysmod/bin` ld flag to fix this. Also replace any reference in this README of `lua_shared.so` with `lua_shared_srv.so` for servers!

You can also try messing with `LD_LIBRARY_PATH` when running `srcds_linux`.

`build.rs`:

```rust
fn main() {
    println!("cargo:rustc-link-search=native=.");
    println!("cargo:rustc-cdylib-link-arg=-Wl,-rpath,./garrysmod/bin");
    println!("cargo:rustc-cdylib-link-arg=-l:lua_shared.so");
}
```
