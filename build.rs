use cc::Build;

fn main() {
        
    // Use the `cc` crate to build a C file and statically link it.    
    Build::new()
        .file("src/native/main.c")
        .flag("/Isrc/native/zlib/zlib-master")                
        .compile("native");
}