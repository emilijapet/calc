/* File: build.rs */
extern crate bindgen;
extern crate cc;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the build crate whenever the source changes
    println!("cargo:rerun-if-changed=libepmath/epmath.c");

	// Build static library
    cc::Build::new()
        .file("libepmath/epmath.c")
        .compile("epmath");

    // Tell cargo to invalidate the build crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=libepmath/epmath.h");
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("libepmath/epmath.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the src/bindings.rs file.
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

