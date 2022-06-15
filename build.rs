use std::env;
use std::path::PathBuf;

const MSVC: &str = "msvc";
const MINGW: &str = "gnu-mingw";

fn main() {
    let target = env::var("TARGET").expect("valid TARGET defined");
    if target.contains("pc-windows") {
        let manifest_dir = PathBuf::from(
            env::var("CARGO_MANIFEST_DIR").expect("valid CARGO_MANIFEST_DIR defined"),
        );
        let mut lib_dir = manifest_dir.join("lib").clone();
        let mut dll_dir = manifest_dir.join("lib").clone();
        
        if target.contains("msvc") {
            lib_dir.push(MSVC);
            dll_dir.push(MSVC);
        } else if target.contains("gnu") {
            lib_dir.push(MINGW);
            dll_dir.push(MINGW);
        } else {
            panic!("{} target unsupported", target);
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
        println!(
            "cargo:rustc-link-search=LD_LIBRARY_PATH={}",
            dll_dir.display()
        );
        println!("cargo:rustc-link-search=LIBRARY_PATH={}", dll_dir.display());
        for entry in std::fs::read_dir(&dll_dir).unwrap_or_else(|err| {
            panic!("can't read dll directory: {}, {}", dll_dir.display(), err)
        }) {
            let entry_path = dbg!(entry.expect("invalid fs entry").path());
            let file_name_result = entry_path.file_name();
            let mut new_file_path = manifest_dir.clone();
            if let Some(file_name) = file_name_result {
                let file_name = dbg!(file_name.to_str().expect("valid filename"));
                if file_name.ends_with(".dll") {
                    new_file_path.push(file_name);
                    std::fs::copy(&entry_path, &new_file_path).unwrap_or_else(|err| {
                        panic!(
                            "can't copy dll directory: {} to {}, {}",
                            entry_path.display(),
                            new_file_path.display(),
                            err
                        )
                    });
                }
            }
        }
    }
}
