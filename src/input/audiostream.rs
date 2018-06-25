use byteorder::*;
use std::io::Bytes;
use std::io::Cursor;
use std::io::Error;
use std::io::Read;

const BUFFER_SIZE: usize = 8192;

pub struct BruteForceStream<T: Read> { 
    // the stream that we will read bytes from
    stream: T,
    // The buffer that we're going to read into from the file
    u8buffer: [u8; BUFFER_SIZE],
    eof: bool, 
    // and the buffer to write converted floats into
    f32buffer: [f32; BUFFER_SIZE/4], 
    floats_read: usize,
    // the index which next() will read a float from 
    read_index: usize, 
}

impl<T:Read> BruteForceStream<T> { 
    #[flame]
    pub fn from_stream(stream: T) -> BruteForceStream<T> {
        BruteForceStream {
            stream: stream,
            u8buffer: [0; BUFFER_SIZE],
            eof: false, 
            f32buffer: [0.0; BUFFER_SIZE/4], 
            floats_read: 0, 
            read_index: 0
        }
    }
}

impl<T: Read> Iterator for BruteForceStream<T> {
    type Item = f32;

    fn next(self: &mut Self) -> Option<f32> { 
        if self.read_index < self.floats_read {
            // if we have data waiting, and ready, simply return it
            let result = Some(self.f32buffer[self.read_index]);
            self.read_index += 1;
            return result;
        }else{
            // otherwise, we need to read in more data! 
            // start off by checking to see if our last read was EOF
            if self.eof == true { 
                return None
            }
            // otherwise, try reading a new set of data into our u8 buffer
            let read = self.stream.read(&mut self.u8buffer[..]);
            // pattern match on the read to see the result
            match read { 
                Ok(bytes_read) => { 
                    // check for EOF with our new read
                    if bytes_read == 0 { 
                        self.eof = true;
                        return None
                    }else {
                        // get that many bytes as a slice! 
                        let mut u8slice = &self.u8buffer[..bytes_read];
                        // convert them to a float
                        // and update the floats_read and read_index
                        let conversion_result = u8slice.read_f32_into::<LittleEndian>(&mut self.f32buffer[..bytes_read/4]);
                        self.read_index = 0; 
                        self.floats_read = bytes_read/4;

                        //finally, do some error checking
                        match conversion_result { 
                            Ok(_) => return Some(self.f32buffer[self.read_index]), 
                            Err(e) => { 
                                println!("Encountered conversion error: {:?}", e);
                                return None
                            }
                        }

                    }
                },
                Err(error) => {
                    println!("Encountered byte read error: {:?}", error); 
                    return None
                }
            }
        }
    }
}

/*
    Float stream gives us a way to convert buffers into an iterator.
    Essentially, we can treat a FloatStream as an iterator until it returns
    `None`, at which point we can choose to refresh it, and continue if we want
*/
#[derive(Debug)]
pub struct FloatStream {
    size: usize,
    index: usize,
    buffer: Vec<f32>,
}

impl FloatStream {
    pub fn with_capacity(capacity: usize) -> FloatStream {
        FloatStream {
            size: capacity,
            index: capacity + 1, // we need data by default
            buffer: Vec::with_capacity(capacity),
        }
    }

    pub fn refresh(self: &mut FloatStream, buffer: &mut ByteBuffer) -> Result<(), ()> {
        println!("FloatStream.refresh");
        let mut u8slice = &buffer.buffer[..buffer.size];
        let convert_status =
            u8slice.read_f32_into::<LittleEndian>(&mut self.buffer[..u8slice.len() / 4]);

        match convert_status {
            Ok(_) => {
                // all good. reset the pointer, and set the size accordingly
                self.index = 0;
                self.size = buffer.size / 4;
                Ok(())
            }
            Err(_) => Err(()),
        }
    }
}

impl Iterator for FloatStream {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        println!("FloatStream.next");
        // if we still have floats left to send out
        if self.index < self.size {
            self.index += 1;
            Some(self.buffer[self.index])
        } else {
            None
        }
    }
}

/*
    ByteBuffers give us a way to read, cleanly, from a file into a buffer, 
    keeping track of the number of bytes read along the way
*/

#[derive(Debug)]
pub struct ByteBuffer {
    pub size: usize,
    pub buffer: Vec<u8>,
}

impl ByteBuffer {
    pub fn with_capacity(capacity: usize) -> ByteBuffer {
        ByteBuffer {
            size: capacity,
            buffer: Vec::with_capacity(capacity),
        }
    }

    pub fn read_from_stream<T: Read>(self: &mut ByteBuffer, stream: &mut T) -> Result<(), ()> {
        println!("read_from_stream");

        let read = stream.read(&mut self.buffer[..]);

        println!("Read returned: {:?}", read);
        match read {
            Ok(bytes) => {
                println!("Read {} bytes", bytes);
                self.size = bytes;
                Ok(())
            }
            Err(_) => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct AudioStream<T: Read> {
    // stream: T,
    bytes: Bytes<T>,
    bytebuffer: ByteBuffer,
    f32stream: FloatStream,
}

impl<T: Read> AudioStream<T> {
    pub fn from_stream(stream: T) -> AudioStream<T> {
        let bytes = 8192;
        AudioStream {
            // stream: stream,
            bytes: stream.bytes(),
            // initialise a default buffer size of bytes
            bytebuffer: ByteBuffer::with_capacity(bytes),
            // initialise a similar default buffer size, but of floats
            // (i.e. 4x fewer)
            f32stream: FloatStream::with_capacity(bytes / 4),
        }
    }

    fn unwrap_byte(wrapped_byte: Option<Result<u8, Error>>) -> Option<u8> {
        match wrapped_byte {
            Some(result) => {
                // now that we have a result, see if it's okay or not
                match result {
                    Ok(byte) => Some(byte), // simply return it
                    Err(e) => {
                        println!("Error in unwrapping byte: {:?}", e);
                        None
                    }
                }
            }
            None => None,
        }
    }
}

impl<T: Read> Iterator for AudioStream<T> {
    type Item = f32;

    // SO SLOW
    fn next(&mut self) -> Option<f32> {
        // println!("Next!");
        // get an iterator to the first four elements of the bytes
        let byte1 = Self::unwrap_byte(self.bytes.next())?;
        let byte2 = Self::unwrap_byte(self.bytes.next())?;
        let byte3 = Self::unwrap_byte(self.bytes.next())?;
        let byte4 = Self::unwrap_byte(self.bytes.next())?;
        let mut rdr = Cursor::new(vec![byte1, byte2, byte3, byte4]);
        let float = rdr.read_f32::<LittleEndian>().unwrap();
        // println!("Found: {:?}", float);
        Some(float)
    }

    // fn next(&mut self) -> Option<f32> {
    //     println!("AudioStream.next");
    //     // try to get some data from our float_buffer. if it returns none,
    //     // then refresh it with data, and try again
    //     let peek = self.f32stream.next();
    //     println!("Peek returned: {:?}", &peek);
    //     match peek {
    //         Some(f) => {
    //             // if peeking gives us data, we can just return that, and trust
    //             // the floatstream to maintain state
    //             Some(f)
    //         },
    //         None => {
    //             println!("Peek gave None");
    //             // however, if we get nothing, we need to a) refresh the buffer
    //             // then b) try again
    //             // to begin with, read some data into the byte buffer
    //             self.bytebuffer.read_from_stream(&mut self.stream).unwrap();

    //             if self.bytebuffer.size == 0 {
    //                 // if we've read zero bytes, give up, as we're at EOF
    //                 None
    //             } else {
    //                 // otherwise pass it into the floatstream, and try again
    //                 self.f32stream.refresh(&mut self.bytebuffer).unwrap();
    //                 self.f32stream.next()
    //             }
    //         }
    //     }
    // }
}
