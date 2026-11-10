use std::env;
use std::path::PathBuf;

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let project_root = manifest.parent().unwrap();
    let wiiuse_dir = project_root.join("vendor/wiiuse");
    let wiiuse_src = wiiuse_dir.join("src");

    let header = manifest.join("src/wiiuse_shim.h");
    let shim = manifest.join("src/wiiuse_shim.c");

    if !wiiuse_dir.join("CMakeLists.txt").exists() {
        panic!(
            "wiiuse submodule missing. Run: \
             git submodule update --init --recursive"
        );
    }

    println!("cargo:rerun-if-changed={}", header.display());
    println!("cargo:rerun-if-changed={}", shim.display());
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
        .include(&wiiuse_src)
        .include(manifest.join("src"))
        .warnings(true)
        .compile("wiiuse_shim");

    // Link the installed wiiuse library.
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );

    println!("cargo:rustc-link-lib=static=wiiuse");

    // Platform-specific native dependencies.
    match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "macos" => {
            println!("cargo:rustc-link-lib=framework=IOBluetooth");
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=Foundation");
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

        other => {
            panic!("Unsupported target OS: {}", other);
        }
    }

    // Generate Rust bindings for the shim only.
    let bindings = bindgen::Builder::default()
        .header(header.to_string_lossy())
        .allowlist_function("rust_wiiuse_.*")
        .generate()
        .expect("Unable to generate wiiuse bindings");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Unable to write bindings");
}
