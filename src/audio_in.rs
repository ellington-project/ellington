use byteorder::*;

use std::fs::File;
use std::io::Read;

pub struct AudioBuffer(pub Vec<f32>);

impl AudioBuffer {

    #[flame]
    pub fn from_stream<T: Read>(mut stream: T) -> AudioBuffer {
        const SAMPLES: usize = 8192;

    let mut i8buffer = [0; SAMPLES];

    let mut f32buffer: [f32; SAMPLES / 4] = [0.0; SAMPLES / 4];

    let mut final_vec: Vec<f32> = Vec::new();

    loop {
        // read some samples into the buffer
        let read = stream.read(&mut i8buffer[..]);
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
    AudioBuffer(final_vec)
    }

    #[flame]
    pub fn from_file(filename: &String) -> AudioBuffer {
        let mut f = File::open(filename).unwrap();
        AudioBuffer::from_stream(f)
    }
}
