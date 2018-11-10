use nom;
use regex::Regex;
use serde_json;
use std::collections::BTreeMap;

pub type Algorithm = String;
pub type Bpm = i64;

#[derive(Debug)]
pub enum UpdateError {
    NoDataInComment,
    FailedToSerialise,
}
pub type UpdateResult<T> = Result<T, UpdateError>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct EllingtonData {
    pub algs: BTreeMap<Algorithm, Bpm>, // pub algs: Vec<BpmInfo>
}

impl EllingtonData {
    pub fn empty() -> EllingtonData {
        EllingtonData {
            algs: BTreeMap::new(),
        }
    }

    pub fn with_algorithm(a: Algorithm, b: Bpm) -> EllingtonData {
        let mut map = BTreeMap::new();
        map.insert(a, b);
        EllingtonData { algs: map }
    }

    pub fn format(self: &Self, minimal: bool) -> UpdateResult<String> {
        let mut s = String::new();
        s.push_str("[ed|");

        let mut first = true;

        for (algorithm, bpm) in self.algs.iter() {
            if first {
                first = false;
            } else {
                s.push_str(",");
            }
            if minimal {
                s.push_str(&format!("{}~{}", algorithm.chars().next().unwrap(), bpm));
            } else {
                s.push_str(&format!(" {}~{}", algorithm, bpm));
            }
        }

        if minimal {
            s.push_str("|]");
        } else {
            s.push_str(" |]");
        }

        Ok(s)
    }

    pub fn as_json(self: &Self) -> Option<String> {
        serde_json::to_string(self).ok()
    }

    pub fn from_json<S: Into<String>>(json: S) -> Option<EllingtonData> {
        serde_json::from_str(json.into().as_str()).ok()
    }

    fn regex() -> &'static Regex {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[ed\|(.*)\|\]").unwrap();
        }
        &RE
    }

    named!(parse_ed_fragment<&str, Vec<(&str, &str)>>,
        terminated!(preceded!(tag_s!("[ed|"),
        separated_list!(
            tag_s!(","),
            separated_pair!(
            ws!(nom::alpha),
            tag_s!("~"), 
            ws!(nom::digit)
            )
        )), tag_s!("|]"))
    );

    // #[flame]
    pub fn parse(comment: &String) -> Option<EllingtonData> {
        let captures = Self::regex().captures(comment.as_str())?;

        // get the first capture, and try to parse it
        match Self::parse_ed_fragment(captures.get(0)?.as_str()) {
            Ok((_, pairs)) => {
                let mut map = BTreeMap::new();
                for (algorithm, bpm) in pairs {
                    // unwrapping should be safe here, as we've already parsed
                    // digits, which we know should form an int!
                    map.insert(String::from(algorithm), bpm.parse::<i64>().unwrap());
                }

                Some(EllingtonData { algs: map })
            }
            _ => {
                println!("Failed to parse ellington data from comment!");
                None
            }
        }
    }

    // #[flame]
    pub fn update_data(
        self: &Self,
        comment: &String,
        append: bool,
        minimal: bool,
    ) -> UpdateResult<String> {
        let serialised = self.format(minimal)?;

        // test to see if there is any ellington data in the first place...
        let new_comment = match Self::regex()
            .captures(comment.as_str())
            .and_then(|captures| captures.get(1))
        {
            Some(e) => {
                info!("Found ellington data {:?} in comment.", e);
                Self::regex()
                    .replace(comment.as_str(), serialised.as_str())
                    .to_string()
            }
            None => {
                if append {
                    info!("Appending data, none found in comment");
                    format!("{} {}", comment, serialised)
                } else {
                    return Err(UpdateError::NoDataInComment);
                }
            }
        };
        Ok(new_comment)
    }

    // clear ellington data from a string, returning the new string
    pub fn clear_data(comment: &String) -> UpdateResult<String> {
        // test to see if there is any json data in the first place...
        match Self::regex()
            .captures(comment.as_str())
            .and_then(|captures| captures.get(1))
        {
            Some(e) => info!("Found ellington data {:?} in comment.", e),
            None => return Err(UpdateError::NoDataInComment),
        }

        let result = Self::regex().replace(comment.as_str(), "");

        Ok(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::UpdateError::*;
    use super::*;

    #[test]
    fn serialise() {
        let ed = EllingtonData::with_algorithm(String::from("TestAlg"), 842);
        let fm = ed.format(false);
        match fm {
            Ok(s) => assert_eq!(s, "[ed| TestAlg~842 |]"),
            Err(_) => assert!(false),
        }
    }

    mod deserialise {
        use super::*;

        #[test]
        fn simple() {
            let ed = EllingtonData::with_algorithm("TestAlg".to_string(), 842);
            let deser = EllingtonData::parse(&"[ed| TestAlg~842 |]".to_string());
            match deser {
                Some(e) => assert_eq!(ed, e),
                None => assert!(false),
            }
        }

        #[test]
        fn post() {
            let ed = EllingtonData::with_algorithm("TestAlg".to_string(), 842);
            let deser = EllingtonData::parse(
                &"Some, tags, [ed. or other, data [ed| TestAlg~842 |]".to_string(),
            );
            match deser {
                Some(e) => assert_eq!(ed, e),
                None => assert!(false),
            }
        }

        #[test]
        fn pre() {
            let ed = EllingtonData::with_algorithm("TestAlg".to_string(), 842);
            let deser = EllingtonData::parse(
                &"[ed| TestAlg~842 |] Some, tags, [ed. or other, data".to_string(),
            );
            match deser {
                Some(e) => assert_eq!(ed, e),
                None => assert!(false),
            }
        }

        #[test]
        fn mid() {
            let ed = EllingtonData::with_algorithm("TestAlg".to_string(), 842);
            let deser = EllingtonData::parse(
                &"Some, tags, [ed. [ed| TestAlg~842 |] or other, data".to_string(),
            );
            match deser {
                Some(e) => assert_eq!(ed, e),
                None => assert!(false),
            }
        }
    }

    mod clear {
        use super::*;
        #[test]
        fn empty() {
            let comment: String = "chuggy, swinging, [ed, more data".to_string();
            match EllingtonData::clear_data(&comment) {
                Ok(updated) => assert_eq!(comment, updated),
                Err(NoDataInComment) => println!("Correct result - no data found!"),
                Err(FailedToSerialise) => {
                    panic!("Failed to serialise ellington data from comment.")
                }
            }
        }

        #[test]
        fn post() {
            let comment: String = "chugging, swinging, [ed, [ed| naive~1842 |]".to_string();
            let expected: String = "chugging, swinging, [ed, ".to_string();

            match EllingtonData::clear_data(&comment) {
                Ok(updated) => {
                    // initially check that the data isn't the same.
                    assert_ne!(comment, updated);
                    // now, check that it's what we expect.
                    assert_eq!(updated, expected);
                }
                Err(NoDataInComment) => panic!("Failed to parse ellington data from comment."),
                Err(FailedToSerialise) => {
                    panic!("Failed to serialise ellington data from comment.")
                }
            }
        }

        #[test]
        fn pre() {
            let comment: String = "[ed| naive~1842 |] chugging, swinging, [ed,".to_string();
            let expected: String = " chugging, swinging, [ed,".to_string();

            match EllingtonData::clear_data(&comment) {
                Ok(updated) => {
                    // initially check that the data isn't the same.
                    assert_ne!(comment, updated);
                    // now, check that it's what we expect.
                    assert_eq!(updated, expected);
                }
                Err(NoDataInComment) => panic!("Failed to parse ellington data from comment."),
                Err(FailedToSerialise) => {
                    panic!("Failed to serialise ellington data from comment.")
                }
            }
        }

        #[test]
        fn mid() {
            let comment: String = "chugging, [ed| naive~1842 |] swinging, [ed,".to_string();
            let expected: String = "chugging,  swinging, [ed,".to_string();

            match EllingtonData::clear_data(&comment) {
                Ok(updated) => {
                    // initially check that the data isn't the same.
                    assert_ne!(comment, updated);
                    // now, check that it's what we expect.
                    assert_eq!(updated, expected);
                }
                Err(NoDataInComment) => panic!("Failed to parse ellington data from comment."),
                Err(FailedToSerialise) => {
                    panic!("Failed to serialise ellington data from comment.")
                }
            }
        }
    }

    // <<<<<<< HEAD
    //     #[test]
    //     fn append_empty_comment() {
    //         let ed = EllingtonData::with_algorithm("TestAlg".to_string(), 842);
    //         let comment: String = "".to_string();
    //         let expected: String = "[ed| TestAlg~842 |]".to_string();

    //         match ed.update_data(&comment, true) {
    //             Ok(new_comment) => {
    //                 assert_eq!(new_comment, expected);
    //             }
    //             Err(UpdateError::NoDataInComment) => panic!(
    //                 "No data in comment path should not occur! We requested appending behaviour!"
    //             ),
    //             Err(_) => panic!("Some other error occurred!"),
    //         }
    //     }
    // =======
    mod update {}

}
