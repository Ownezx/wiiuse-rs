use std::env;
use std::path::{Path, PathBuf};

fn require_file(path: &Path) {
    if !path.is_file() {
        panic!(
            "required wiiuse-rs source file is missing: {}",
            path.display()
        );
    }
    println!("cargo:rerun-if-changed={}", path.display());
}

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let wiiuse_dir = manifest.join("vendor/wiiuse");
    let wiiuse_src = wiiuse_dir.join("src");

    let header = manifest.join("src/wiiuse_shim.h");
    let shim = manifest.join("src/wiiuse_shim.c");
    let state_shim = manifest.join("src/wiiuse_state_shim.c");

    if !wiiuse_dir.join("CMakeLists.txt").is_file() {
        panic!("wiiuse source missing: {}", wiiuse_dir.display());
    }
    require_file(&header);
    require_file(&shim);
    require_file(&state_shim);
    println!(
        "cargo:rerun-if-changed={}",
        wiiuse_dir.join("CMakeLists.txt").display()
    );
    println!("cargo:rerun-if-changed={}", wiiuse_src.display());

    // Build upstream wiiuse.
    let dst = cmake::Config::new(&wiiuse_dir)
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_EXAMPLE", "OFF")
        .define("BUILD_EXAMPLE_SDL", "OFF")
        .define("INSTALL_EXAMPLES", "OFF")
        .define("BUILD_TESTING", "OFF")
        .build();

    // Compile the C shim.
    cc::Build::new()
        .file(&shim)
        .file(&state_shim)
        .include(&wiiuse_src)
        .include(manifest.join("src"))
        .warnings(true)
        .compile("wiiuse_shim");

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=wiiuse");

    match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "macos" => {
            println!("cargo:rustc-link-lib=framework=IOBluetooth");
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=Foundation");
            println!("cargo:rustc-link-lib=framework=CoreBluetooth");
            println!("cargo:rustc-link-lib=framework=IOKit");
        }
        "linux" => {
            println!("cargo:rustc-link-lib=bluetooth");
            println!("cargo:rustc-link-lib=m");
            println!("cargo:rustc-link-lib=rt");
        }
        "windows" => {
            println!("cargo:rustc-link-lib=ws2_32");
            println!("cargo:rustc-link-lib=setupapi");
        }
        other => panic!("Unsupported target OS: {other}"),
    }

    // Generate bindings for the private shim API only. They are included only
    // by src/ffi.rs and are not re-exported to users of this crate.
    let bindings = bindgen::Builder::default()
        .header(header.to_string_lossy())
        .allowlist_function("rust_wiiuse_.*")
        .allowlist_type("rust_wiiuse_state_t")
        .generate()
        .expect("Unable to generate wiiuse bindings");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Unable to write bindings");
}
