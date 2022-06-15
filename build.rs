use std::env;
use std::path::PathBuf;

const MSVC: &str = "lib/msvc";
const MINGW: &str = "lib/gnu-mingw";

fn main() {
    let target = env::var("TARGET").expect("valid TARGET defined");
    if target.contains("pc-windows") {
        let manifest_dir = PathBuf::from(
            env::var("CARGO_MANIFEST_DIR").expect("valid CARGO_MANIFEST_DIR defined"),
        );
        let mut lib_dir = manifest_dir.clone();
        let mut dll_dir = manifest_dir.clone();
        if target.contains("msvc") {
            lib_dir.push(MSVC);
            dll_dir.push(MSVC);
        } else if target.contains("gnu") {
            lib_dir.push(MINGW);
            dll_dir.push(MINGW);
        } else {
            eprintln!("{} target unsupported", target);
            return;
        }
        lib_dir.push("lib");
        dll_dir.push("dll");
        if target.contains("x86_64") {
            lib_dir.push("64");
            dll_dir.push("64");
        } else {
            lib_dir.push("32");
            dll_dir.push("32");
        }
        println!("cargo:rustc-link-search=all={}", lib_dir.display());
        for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL directory") {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path = manifest_dir.clone();
            if let Some(file_name) = file_name_result {
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".dll") {
                    new_file_path.push(file_name);
                    std::fs::copy(&entry_path, new_file_path.as_path())
                        .expect("Can't copy from DLL directory");
                }
            }
        }
    }
}
