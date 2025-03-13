use cc::Build;

fn main() {

     // Specify the directory where `clap.lib` is located.
    println!("cargo:rustc-link-search=src\\native");
    // Tell rustc to link with the `clap` library (assuming `clap.lib` is the import library for `clap.dll`).
    println!("cargo:rustc-link-lib=dylib=native-server");

    // Use the `cc` crate to build a C file and statically link it.    
    Build::new()    
        .file("src/native/main.c")         
        .compile("native");
}