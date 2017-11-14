extern crate bindgen;

use std::env;
use std::process::Command;
use std::fs::File;
use std::path::PathBuf;

fn main() {
    let status = Command::new("make")
        .arg("build/lib/libtransistor.nro.a")
        .current_dir("libtransistor")
        .status().unwrap();
    if !status.success() {
        panic!("Make failed");
    }

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-lib=static=transistor.nro");
    println!("cargo:rustc-link-search=native={}/libtransistor/build/lib", dir);
    println!("cargo:rustc-link-search=native={}/libtransistor/newlib/aarch64-none-switch/newlib", dir);

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    /*
     * let status = Command::new("rustup").args(&["run", "nightly", "bindgen"])
        .args(&["--blacklist-type", "u(8|16|32|64)"])
        .arg("--use-core")
        .args(&["--ctype-prefix", "::libc"])
        .arg("libtransistor/include/libtransistor/nx.h")
        .args(&["--", "-nostdinc"])
        .args(&["-isystem", "/usr/lib/clang/5.0.0/include"])
        .args(&["-isystem", "libtransistor/newlib/newlib/libc/include"])
        .args(&["-isystem", "libtransistor/newlib/newlib/libc/sys/switch/include"])
        .arg("-Ilibtransistor/include")
        .stdout(File::create(out_path.join("bindings.rs")).unwrap())
        .status().unwrap();
       if !status.success() {
           panic!("bindgen failed");
       }
    */


    let bindings = bindgen::Builder::default()
        .header("libtransistor/include/libtransistor/nx.h")
        // Don't use host headers, to make sure we're building against newlib
        .clang_arg("-nostdinc")
        // Include the newlib/transistor headers, and the clang builtin headers
        .clang_args(&["-isystem", "/usr/lib/clang/5.0.0/include"])
        .clang_args(&["-isystem", "libtransistor/newlib/newlib/libc/include"])
        .clang_args(&["-isystem", "libtransistor/newlib/newlib/libc/sys/switch/include"])
        .clang_arg("-Ilibtransistor/include")
        // We don't need to define those types, rust has them already anyways.
        // Blacklisting avoids a bug in bindgen where it creates cyclic references
        // (pub type u8 = u8)
        .blacklist_type("u(8|16|32|64)")
        .blacklist_type(".*va_list")
        .blacklist_type("dbg_vs?n?printf(cb)?")
        .use_core()
        .ctypes_prefix("cty")
        .rustfmt_bindings(true)
        .rustfmt_configuration_file(None)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // TODO: compile the damned thing
}
