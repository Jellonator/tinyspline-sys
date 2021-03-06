use bindgen;
use cc;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=tinyspline/src/tinyspline.h");
    println!("cargo:rerun-if-changed=tinyspline/src/tinyspline.c");
    println!("cargo:rerun-if-changed=tinyspline/src/parson.h");
    println!("cargo:rerun-if-changed=tinyspline/src/parson.c");

    cc::Build::new()
        .file("tinyspline/src/parson.c")
        .file("tinyspline/src/tinyspline.c")
        .compile("libtinyspline");
    
    let bindings = bindgen::Builder::default()
        .header("tinyspline/src/parson.h")
        .header("tinyspline/src/tinyspline.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate_comments(false)
        // size_t is blacklisted because otherwise it will be set to either
        // 'u64' or 'u32'. This results in code that isn't cross-compatible
        // between platforms with different bit sizes.
        .blacklist_type("size_t")
        .generate()
        .expect("Unable to generate bindings for tinyspline.");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}