use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let ellington_dir = env::args().nth(1).unwrap_or("./".to_string());

    let build_dir = Path::new(&ellington_dir)   
    .join("target")
    .join("release")
    .join("build");
    
    let taglibdir = find_directory_containing(&build_dir, "talamel", "out").unwrap()
        .join("out")
        .join("build")
        .join("taglib")
        .join("install")
        .join("lib");

    println!("Found taglib directory {:?}", taglibdir);

    for entry in std::fs::read_dir(taglibdir).expect("Cannot read from taglib directory") { 
        println!("entry: {:?}", entry);
        let path = entry.expect("Invalid fs path.").path(); 
        match path.file_name() { 
            Some(name) => {
                let name = name.to_str().unwrap();
                if name.contains("tag.dll") {
                    println!("Found shared library: {:?}", name);
                    let dest = Path::new(&ellington_dir).join(name);
                    println!("Copying from\n\t{:?}\nto\n\t{:?}", path, dest);
                    match std::fs::copy(&path, dest) {
                        Ok(b) => {
                            println!("Successfully copied {:?} bytes.", b);
                        },
                        Err(e) => {
                            println!("Encountered error: {:?} while copying.", e);
                        }
                    };
                }
            }, 
            _ => {}
        }
    }

}

fn find_directory_containing(parent: &Path, s: &str, subdir: &str) -> Option<PathBuf> { 
    for entry in std::fs::read_dir(parent.clone()).expect("Can't read parent directory") { 
        let path = entry.expect("Invalid fs path.").path(); 
        match path.file_name() { 
            Some(name) => {
                let name = name.to_str().unwrap();
                if name.contains(s) {         
                            if path.join(subdir).exists(){ 
                                return Some(path.to_path_buf());
                            }
                }else {
                    
                }
            },
            None => panic!("Can't find ellington directory!")
        }
    }
    None
}
