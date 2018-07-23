use byteorder::*;

use std::io::Read;

pub struct AudioBuffer {
    buffer: Vec<f32>,
    ix: usize,
}

#[allow(dead_code)]
impl AudioBuffer {
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
        AudioBuffer {
            buffer: final_vec,
            ix: 0,
        }
    }
}

impl Iterator for AudioBuffer {
    type Item = f32;

    fn next(self: &mut AudioBuffer) -> Option<f32> {
        if self.ix < self.buffer.len() {
            let result = self.buffer[self.ix];
            self.ix += 1;
            Some(result)
        } else {
            None
        }
    }
}
