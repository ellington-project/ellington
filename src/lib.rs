/*
    libellington - the core library + functionality for the ellington tool.
*/

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate nom;

extern crate byteorder;

// extern crate id3;
// extern crate itertools;
// extern crate memmap;
extern crate percent_encoding;
extern crate plist;
extern crate rand;

extern crate url;
extern crate walkdir;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate regex;
#[macro_use]
extern crate lazy_static;

// extern crate taglib;

// pub mod actions;
pub mod estimators;
pub mod library;
pub mod shelltools;

pub fn trueish() -> bool {
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn tautology_internal() {
        assert!(super::trueish());
    }
}
