/*
    libellington - the core library + functionality for the ellington tool.
*/

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate nom;

extern crate byteorder;

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

extern crate talamel;

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

    #[test]
    fn serialise_simple() {
        use library::ellingtondata::*;
        let ed = EllingtonData::with_algorithm("TestAlg".to_string(), 842);
        let fm = ed.format();
        match fm {
            Ok(s) => assert_eq!(s, " [ed| TestAlg~842 |]"),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn deserialise_simple() {
        use library::ellingtondata::*;
    }
}
