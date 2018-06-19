use byteorder::*;
use std::io::Cursor;
use memmap::Mmap;

use std::io::*;
use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::path::Path;

use std::process::Command;
use std::process::Stdio;

fn call_and_read(command: &mut Command) -> Vec<u8> {
    command.output().expect("Failed to execute process").stdout
}

fn read_u32_f32_into(data: &Vec<u8>) -> Result<Vec<f32>> {
    // assert that it's the right size
    assert_eq!(data.len() % 4, 0);

    let mut datac = Cursor::new(data);
  
    let mut vf32 : Vec<f32> = Vec::with_capacity(data.len() / 4);
    datac.read_f32_into::<LittleEndian>(&mut vf32)?;

    return Ok(vf32);
}

fn read_binary_file(filename: &Path) -> Vec<f32> {
    let mut f = File::open(filename).unwrap();

    // create a buffer to read results into:
    const SAMPLES: usize = 8192;

    let mut i8buffer = [0; SAMPLES];

    let mut f32buffer: [f32; SAMPLES / 4] = [0.0; SAMPLES / 4];

    let mut final_vec: Vec<f32> = Vec::new();

    loop {
        // read some samples into the buffer
        let read = f.read(&mut i8buffer[..]);
        match read {
            Ok(bytes) => {
                if bytes == 0 {
                    break;
                } else {
                    // get that many bytes as a slice
                    let mut i8slice = &i8buffer[..bytes];
                    let r = i8slice.read_f32_into::<LittleEndian>(&mut f32buffer[..bytes / 4]);

                    match r {
                        Ok(_) => {
                            final_vec.extend_from_slice(&f32buffer[..bytes / 4]);
                        }
                        Err(e) => println!("Encountered convert error {:?}", e),
                    }
                }
            }
            Err(error) => println!("Encountered read error: {:?}", error),
        }
    }
    return final_vec;
}

// Ignore this one for now...
fn mmap_binary_file(filename: &Path) -> () {

    let file = File::open(filename).expect("failed to open the file");

    let mmap = unsafe { Mmap::map(&file).expect("failed to map the file") };

    // turn the mapped file into an array of f32
    let bytes_length = mmap.len();

    // make sure that we can turn it into floating point numbers
    assert_eq!(bytes_length % 4, 0);

    // let mut f32data : Vec<f32> = Vec::with_capacity(bytes_length/4);
    // &mmap.read_f32_into::<LittleEndian>(&f32data);

    stdout()
        .write_all(&mmap[..])
        .expect("failed to output the file contents");
        //     let file_mmap = Mmap::open_path("./src/book.txt", Protection::Read).unwrap();
        // let bytes: &[u8] = unsafe { file_mmap.as_slice() };
        // let mut p = 0;
        // while p < bytes.len()-CHUNK_SIZE  {
        //     println!("{}:\t{}",p,str::from_utf8(&bytes[p..p+CHUNK_SIZE]).unwrap());
        //     p+=CHUNK_SIZE;
        // }
        // Ok(())

}

pub fn run_sox_and_read_file(mp3: &Path, dat: &Path) -> Vec<f32> {
    // Get the data using the sox command
    let command = format!(
        // "sox -V1 \"{:?}\" -L -r 48000 -e float -b 16 -t raw \"{:?}\"",
        "sox -V1 \"{:?}\" -r 44100 -e float -c 1 -b 16 -t raw \"{:?}\"",
        mp3,
        dat
    );

    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());

    let cmd_result = read_binary_file(dat);

    return cmd_result;
}

// #[cfg(test)]
// mod tests {

//     // import all of our sox stuff
//     use super::*;

//     use std::path::Path;

//     #[test]
//     fn test_read_audio_file() {
//         // First use sox to generate a file from the test data
//         let cargo_root = env!("CARGO_MANIFEST_DIR");
//         let d = Path::new(cargo_root).join("data");

//         let mp3 = Path::new(&d).join("test.mp3");
//         let dat = Path::new(&d).join("test.txt");

//         // get the data using our function
//         let lib_result = read_audio_file(mp3.as_path()).unwrap();

//         let cmd_result = run_sox_and_read_file(mp3.as_path(), dat.as_path());

//         println!(
//             "\tlib_result: {}/{:?}\t\ncmd_result: {}/{:?}",
//             lib_result.len(),
//             &lib_result[lib_result.len() - 16..],
//             cmd_result.len(),
//             &cmd_result[cmd_result.len() - 16..]
//         );

//         assert!(true);
//         // assert_eq!(lib_result, cmd_result);
//     }
// }
