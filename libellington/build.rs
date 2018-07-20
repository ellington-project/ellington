#[cfg(target_os = "linux")]
fn print_linker_arguments() {
    println!("Detected a Linux platform");
    // println!("cargo:rustc-link-lib=static=stdc++");
    println!("cargo:rustc-flags=-l tag_c -l tag -l z -l stdc++");
}

#[cfg(target_os = "macos")]
fn print_linker_arguments() {
    println!("Detected an OSX platform");
    println!("cargo:rustc-flags=-l dylib=z");
    println!("cargo:rustc-flags=-l tag_c -l tag");
    println!("cargo:rustc-link-lib=c++");
}

fn main() {
    println!("Printing linker arguments:");
    print_linker_arguments();
}