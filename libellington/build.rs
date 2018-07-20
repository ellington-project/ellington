#[cfg(target_os = "linux")]
fn print_linker_arguments() {
    println!("Printing linker arguments for Linux");
    println!("cargo:rustc-link-lib=static=stdc++");
    println!("cargo:rustc-flags=-l tag_c -l tag -l z");
}

#[cfg(target_os = "macos")]
fn print_linker_arguments() {
    println!("Printing linker arguments for OSX");
    println!("cargo:rustc-flags=-l dylib=z");
    println!("cargo:rustc-flags=-l tag_c -l tag");
    println!("cargo:rustc-link-lib=c++");
}

fn main() {
    print_linker_arguments();
}