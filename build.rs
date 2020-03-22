const LUA_SHARED_DLL_PATH: &str =
    r#"F:\SteamLibrary\steamapps\common\GarrysMod\bin\win64\lua_shared.dll"#;

use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let bindings = bindgen::Builder::default()
    .header("LuaJIT/src/lua.h")
    // Tell cargo to invalidate the built crate whenever any of the
    // included header files changed.
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .whitelist_var("luaL?_.*")
    .whitelist_type("luaL?_.*")
    .whitelist_function("luaL?_.*")
    .whitelist_var("LUAL?_.*")
    .whitelist_type("LUAL?_.*")
    .whitelist_function("LUAL?_.*")
    .generate()
    .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    let lua_shared_lib_path = out_dir.join("lua_shared.lib");
    let lua_shared_def_path = out_dir.join("lua_shared.def");
    let lua_shared_dll_path = Path::new(LUA_SHARED_DLL_PATH);

    {
        let exports = get_exports(&lua_shared_dll_path);

        let mut lua_shared_def = File::create(&lua_shared_def_path).unwrap();

        writeln!(lua_shared_def, "EXPORTS").unwrap();
        for function_name in exports {
            writeln!(lua_shared_def, "{}", function_name).unwrap();
        }
    }

    assert!(get_tool("lib.exe")
        .arg("/NOLOGO")
        .arg("/MACHINE:x64")
        .arg(format!("/DEF:{}", lua_shared_def_path.display()))
        .arg(format!("/OUT:{}", lua_shared_lib_path.display()))
        .status()
        .unwrap()
        .success());

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=dylib=lua_shared");
}

fn get_tool(name: &str) -> Command {
    cc::windows_registry::find(&env::var("TARGET").unwrap(), name).unwrap()
}

fn get_exports<P: AsRef<Path>>(dll_path: P) -> Vec<String> {
    let dll_path = dll_path.as_ref();

    let output = get_tool("dumpbin.exe")
        .arg("/EXPORTS")
        .arg(dll_path)
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    stdout
        .split("\r\n")
        .map(|line| line.trim())
        .skip_while(|line| line != &"ordinal hint RVA      name")
        .skip(2)
        .take_while(|line| line != &"")
        .map(|line| line.split_whitespace().nth(3).unwrap())
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
}
