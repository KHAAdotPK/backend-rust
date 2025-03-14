use cc::Build;

fn main() {

    // Specify the path to the directory where `native-server.lib` is located
    println!("cargo:rustc-link-search=src\\native");
    // Tell rustc to link with the `native-server.lib` library (assuming `native-server.lib` is the import library for `native-server.dll`)
    println!("cargo:rustc-link-lib=dylib=native-server");

    // Specify the path to the directory where `clap.lib` is located
    println!("cargo:rustc-link-search=argsv-rust\\Debug\\");
    // Tell rustc to link with the `clap.lib` library (assuming `clap.lib` is the import library for `clap.dll`)
    println!("cargo:rustc-link-lib=dylib=clap");
    
    std::fs::copy("argsv-rust\\Debug\\clap.dll", "target\\Debug\\clap.dll").unwrap_or_else(|err| {
        panic!("Failed to copy {} to {}: {}", "argsv-rust\\Debug\\clap.dll", "target\\Debug", err);
    });

    std::fs::copy("argsv-rust\\Debug\\clap.dll", "target\\Debug\\start.dll").unwrap_or_else(|err| {
        panic!("Failed to copy {} to {}: {}", "argsv-rust\\Debug\\start.dll", "target\\Debug", err);
    });

    // Use the `cc` crate to build a C file and statically link it    
    Build::new()    
        .file("src/native/main.c")         
        .compile("native");
}