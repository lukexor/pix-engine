use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changes=build.rs");

    let target = env::var("TARGET").expect("valid TARGET defined");
    if target.contains("pc-windows-msvc") {
        let manifest_dir = PathBuf::from(
            env::var("CARGO_MANIFEST_DIR").expect("valid CARGO_MANIFEST_DIR defined"),
        );
        let out_dir = PathBuf::from(env::var("OUT_DIR").expect("valid CARGO_OUT_DIR defined"));
        let mut lib_dir = manifest_dir.join("lib").join("msvc").join("lib");
        let mut dll_dir = manifest_dir.join("lib").join("msvc").join("dll");

        if target.contains("x86_64") {
            lib_dir.push("64");
            dll_dir.push("64");
        } else {
            lib_dir.push("32");
            dll_dir.push("32");
        }

        println!("cargo:rustc-link-search=native={}", out_dir.display());

        for dir in [lib_dir, dll_dir] {
            for entry in std::fs::read_dir(&dir)
                .unwrap_or_else(|err| panic!("can't read directory: {}, {}", dir.display(), err))
            {
                let entry_path = entry.expect("invalid fs entry").path();
                let file_name_result = entry_path.file_name();
                if let Some(file_name) = file_name_result {
                    let file_name = file_name.to_str().expect("valid filename");
                    let new_file_path = out_dir.join(file_name);
                    std::fs::copy(&entry_path, &new_file_path).unwrap_or_else(|err| {
                        panic!(
                            "can't copy directory: {} to {}, {}",
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
