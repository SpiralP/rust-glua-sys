use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    let bindings = bindgen::Builder::default()
    .header("LuaJIT/src/luajit.h")
    .header("LuaJIT/src/lualib.h")
    .header("LuaJIT/src/lauxlib.h")
    // Tell cargo to invalidate the built crate whenever any of the
    // included header files changed.
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))

    .whitelist_var("lua.*")
    .whitelist_type("lua.*")
    .whitelist_function("lua.*")

    .whitelist_var("LUA.*")
    .whitelist_type("LUA.*")
    .whitelist_function("LUA.*")

    .generate()
    .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    #[cfg(not(feature = "ci"))]
    {
        link();
    }
}

fn link() {
    #[cfg(target_os = "windows")]
    {
        let out_dir = env::var("OUT_DIR").unwrap();
        let out_dir = Path::new(&out_dir);

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

        fn find_lua_shared_dll() -> Option<PathBuf> {
            if let Ok(lua_shared_dll_path) = env::var("LUA_SHARED_DLL_PATH") {
                return Some(PathBuf::from(lua_shared_dll_path));
            }

            let steam_paths: Vec<PathBuf> = [
                r#"C:\Program Files (x86)\Steam"#,
                r#"C:\SteamLibrary"#,
                r#"D:\SteamLibrary"#,
                r#"E:\SteamLibrary"#,
                r#"F:\SteamLibrary"#,
            ]
            .iter()
            .map(|a| a.into())
            .collect();

            for steam_path in steam_paths {
                let lua_shared_dll_path = steam_path
                    .join("steamapps")
                    .join("common")
                    .join("GarrysMod")
                    .join("bin")
                    .join("win64")
                    .join("lua_shared.dll");

                if lua_shared_dll_path.is_file() {
                    return Some(lua_shared_dll_path);
                }
            }

            None
        }

        let lua_shared_dll_path = find_lua_shared_dll().expect(
            "lua_shared.dll couldn't be found! Try setting the environment variable \
             LUA_SHARED_DLL_PATH to where lua_shared.dll is located",
        );
        assert_eq!(lua_shared_dll_path.extension().unwrap(), "dll");
        let lua_shared_file_stem = lua_shared_dll_path.file_stem().unwrap().to_str().unwrap();

        let lua_shared_lib_path = out_dir.join(format!("{}.lib", lua_shared_file_stem));
        let lua_shared_def_path = out_dir.join(format!("{}.def", lua_shared_file_stem));

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
        println!("cargo:rustc-link-lib=dylib={}", lua_shared_file_stem);
    }
}
