
pub struct Reader<T>{ 
    buffer: Vec<T>
}

impl<T> Reader<T> {
    pub fn read_in<S>(source: S) -> Reader<T> { 
        // Read some data from t, into a buffer
        let buffer : Vec<T> = Vec::new();
        //...
        Reader { buffer: buffer }
    }
}

impl<T> Iterator for Reader<T> {
    type Item = T;

    fn next(self: &mut Reader<T>) -> Option<T> {
        unimplemented!()
        // Possible option?
        // buffer.into_iter().next()
    }
}