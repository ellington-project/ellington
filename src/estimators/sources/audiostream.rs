use byteorder::*;
use std::io::Read;

const BUFFER_SIZE: usize = 8;

pub struct AudioStream<T: Read> {
    // the stream that we will read bytes from
    stream: T,
    // The buffer that we're going to read into from the file
    u8buffer: [u8; BUFFER_SIZE],
    eof: bool,
    // and the buffer to write converted floats into
    f32buffer: [f32; BUFFER_SIZE / 4],
    floats_read: usize,
    // the index which next() will read a float from
    read_index: usize,
}

impl<T: Read> AudioStream<T> {
    pub fn from_stream(stream: T) -> AudioStream<T> {
        AudioStream {
            stream: stream,
            u8buffer: [0; BUFFER_SIZE],
            eof: false,
            f32buffer: [0.0; BUFFER_SIZE / 4],
            floats_read: 0,
            read_index: 0,
        }
    }
}

impl<T: Read> Iterator for AudioStream<T> {
    type Item = f32;

    fn next(self: &mut Self) -> Option<f32> {
        if self.read_index < self.floats_read {
            // if we have data waiting, and ready, simply return it
            let result = Some(self.f32buffer[self.read_index]);
            self.read_index += 1;
            return result;
        } else {
            // otherwise, we need to read in more data!
            // start off by checking to see if our last read was EOF
            if self.eof == true {
                return None;
            }
            // otherwise, try reading a new set of data into our u8 buffer
            let read = self.stream.read(&mut self.u8buffer[..]);
            // pattern match on the read to see the result
            match read {
                Ok(bytes_read) => {
                    // check for EOF with our new read
                    if bytes_read == 0 {
                        self.eof = true;
                        return None;
                    } else {
                        // get that many bytes as a slice!
                        let mut u8slice = &self.u8buffer[..bytes_read];
                        // convert them to a float
                        // and update the floats_read and read_index
                        let conversion_result = u8slice
                            .read_f32_into::<LittleEndian>(&mut self.f32buffer[..bytes_read / 4]);
                        self.read_index = 0;
                        self.floats_read = bytes_read / 4;

                        //finally, do some error checking
                        match conversion_result {
                            Ok(_) => return Some(self.f32buffer[self.read_index]),
                            Err(e) => {
                                println!("Encountered conversion error: {:?}", e);
                                return None;
                            }
                        }
                    }
                }
                Err(error) => {
                    println!("Encountered byte read error: {:?}", error);
                    return None;
                }
            }
        }
    }
}
