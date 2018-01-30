extern crate bindgen;

use std::env;
use std::process::Command;
use std::fs::File;
use std::path::PathBuf;

fn main() {
    let dir = env::var("LIBTRANSISTOR_HOME").expect("LIBTRANSISTOR_HOME musts be set");

    let status = Command::new("make")
        .current_dir(&format!("{}", dir))
        .status().expect("Make failed");
    if !status.success() {
        panic!("Make failed");
    }

    // Don't bother linking libc, liblibc takes care of it.
    // TODO: What if liblibc isn't linked ?
    // Solution: have libtransistor-sys use libc always, instead of cty.
    //println!("cargo:rustc-link-lib=static=c");
    // TODO: compiler_builtins takes care of this !
    //println!("cargo:rustc-link-lib=static=clang_rt.builtins-aarch64");
    //println!("cargo:rustc-link-search=native={}/libtransistor/build/compiler-rt/lib/linux", dir);
    println!("cargo:rustc-link-lib=static=transistor.nro");
    println!("cargo:rustc-link-search=native={}/build/lib", dir);

    let out_path = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR must be set"));

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


    // TODO: Avoid generating a ton of useless cruft from liblibc
    let bindings = bindgen::Builder::default()
        .header(format!("{}/include/libtransistor/nx.h", dir))
        // Don't use host headers, to make sure we're building against newlib
        .clang_arg("-nostdlibinc")
        // Include the newlib/transistor headers, and the clang builtin headers
        .clang_args(&["-isystem", "/usr/lib/clang/5.0.0/include"])
        .clang_args(&["-isystem", &format!("{}/newlib/newlib/libc/sys/switch/include", dir)])
        .clang_args(&["-isystem", &format!("{}/build/newlib/aarch64-none-switch/newlib/targ-include", dir)])
        .clang_args(&["-isystem", &format!("{}/newlib/newlib/libc/include", dir)])
        .clang_arg(format!("-I{}/include", dir))
        // We don't need to define those types, rust has them already anyways.
        // Blacklisting avoids a bug in bindgen where it creates cyclic references
        // (pub type u8 = u8)
        .blacklist_type("u(8|16|32|64)")
        .blacklist_type(".*va_list")
        .blacklist_type("dbg_vs?n?printf(cb)?")
        .use_core()
        .ctypes_prefix("cty")
        .rustfmt_bindings(true)
        .generate_comments(true)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
