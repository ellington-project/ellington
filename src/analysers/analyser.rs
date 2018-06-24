/*
  Define a trait for "analysers" to give a standardised interface to the various methods that we might want to employ.
*/

trait Analyser<T> {
    // add code here
    fn analyse(input: T) -> f32 ;
}