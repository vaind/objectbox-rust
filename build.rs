extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the objectbox shared library.
    println!("cargo:rustc-link-lib=objectbox");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("src/objectbox.h")
        // Some settings
        .whitelist_function("obx_.*")
        .whitelist_type("OBX_.*")
        .whitelist_var("OBX_.*")
        .prepend_enum_name(false)
        .derive_copy(false)
        .derive_debug(false)
        .derive_default(false)
        .rustfmt_bindings(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("objectbox-c-bindings.rs"))
        .expect("Couldn't write bindings!");
}
