use nom;
use regex::Regex;
use serde_json;
use std::collections::BTreeMap;
use std::ops;
use types::*;

pub type Algorithm = AlgorithmE;
pub type Bpm = BpmE;

#[derive(Debug)]
pub enum UpdateError {
    NoDataInComment,
    FailedToSerialise,
}
pub type UpdateResult<T> = Result<T, UpdateError>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct EllingtonData {
    pub algs: BTreeMap<Algorithm, Bpm>,
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

    #[flame]
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
                s.push_str(&format!(
                    "{}~{}",
                    algorithm.print().chars().next().unwrap(),
                    bpm
                ));
            } else {
                s.push_str(&format!(" {}~{}", algorithm.print(), bpm));
            }
        }

        if minimal {
            s.push_str("|]");
        } else {
            s.push_str(" |]");
        }

        Ok(s)
    }

    #[flame]
    pub fn format_json(self: &Self) -> Option<String> {
        serde_json::to_string(self).ok()
    }

    #[flame]
    pub fn format_readable(self: &Self) -> Option<String> {
        let mut output = String::new();
        for (alg, tmpo) in &self.algs {
            output += &format!("Algorithm: {}, Tempo: {}\n", alg, tmpo);
        }
        Some(output)
    }

    #[flame]
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
        terminated!(preceded!(tag!("[ed|"),
        separated_list!(
            tag!(","),
            separated_pair!(
                ws!(nom::alpha),
                tag!("~"),
                ws!(
                    alt!(nom::digit| tag!("na"))
                )
            )
        )), tag!("|]"))
    );

    #[flame]
    pub fn parse(comment: &String) -> Option<EllingtonData> {
        let captures = Self::regex().captures(comment.as_str())?;

        // get the first capture, and try to parse it
        match Self::parse_ed_fragment(captures.get(0)?.as_str()) {
            Ok((_, pairs)) => {
                let mut map = BTreeMap::new();
                for (algorithm, bpm) in pairs {
                    // It would be good to think more deeply about what BpmE should do when it fails, as at the moment it always returns "NA", which might not be the best solution...
                    map.insert(AlgorithmE::parse(algorithm), BpmE::parse(bpm));
                }

                Some(EllingtonData { algs: map })
            }
            _ => {
                println!("Failed to parse ellington data from comment!");
                None
            }
        }
    }

    #[flame]
    pub fn update_data(
        self: &Self,
        comment: &String,
        append: UpdateBehaviour,
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
            None => match append {
                UpdateBehaviour::Append => {
                    info!("Appending data, none found in comment");
                    format!("{} {}", comment, serialised)
                }
                UpdateBehaviour::Prepend => {
                    info!("Prepending data, none found in comment");
                    format!("{} {}", serialised, comment)
                }
                _ => {
                    return Err(UpdateError::NoDataInComment);
                }
            },
        };
        Ok(new_comment)
    }

    // clear ellington data from a string, returning the new string
    #[flame]
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

impl ops::Add<EllingtonData> for EllingtonData {
    type Output = EllingtonData;
    fn add(self, rhs: EllingtonData) -> EllingtonData {
        let mut algs: BTreeMap<Algorithm, Bpm> = self.algs.clone();
        algs.append(&mut rhs.algs.clone());
        EllingtonData { algs: algs }
    }
}

#[cfg(test)]
mod tests {
    use super::UpdateError::*;
    use super::*;

    #[test]
    fn serialise() {
        let ed = EllingtonData::with_algorithm(AlgorithmE::parse("unknown"), BpmE::Bpm(842));
        let fm = ed.format(false);
        match fm {
            Ok(s) => assert_eq!(s, "[ed| unknown~842 |]"),
            Err(_) => assert!(false),
        }
    }

    mod deserialise {
        use super::*;
        mod good {
            use super::*;
            #[test]
            fn simple() {
                let ed =
                    EllingtonData::with_algorithm(AlgorithmE::parse("unknown"), BpmE::Bpm(842));
                let deser = EllingtonData::parse(&"[ed| unknown~842 |]".to_string());
                match deser {
                    Some(e) => assert_eq!(ed, e),
                    None => assert!(false),
                }
            }

            #[test]
            fn post() {
                let ed =
                    EllingtonData::with_algorithm(AlgorithmE::parse("unknown"), BpmE::Bpm(842));
                let deser = EllingtonData::parse(
                    &"Some, tags, [ed. or other, data [ed| unknown~842 |]".to_string(),
                );
                match deser {
                    Some(e) => assert_eq!(ed, e),
                    None => assert!(false),
                }
            }

            #[test]
            fn pre() {
                let ed =
                    EllingtonData::with_algorithm(AlgorithmE::parse("unknown"), BpmE::Bpm(842));
                let deser = EllingtonData::parse(
                    &"[ed| unknown~842 |] Some, tags, [ed. or other, data".to_string(),
                );
                match deser {
                    Some(e) => assert_eq!(ed, e),
                    None => assert!(false),
                }
            }

            #[test]
            fn mid() {
                let ed =
                    EllingtonData::with_algorithm(AlgorithmE::parse("unknown"), BpmE::Bpm(842));
                let deser = EllingtonData::parse(
                    &"Some, tags, [ed. [ed| unknown~842 |] or other, data".to_string(),
                );
                match deser {
                    Some(e) => assert_eq!(ed, e),
                    None => assert!(false),
                }
            }
        }

        mod bad {
            use super::*;
            #[test]
            fn simple() {
                let ed = EllingtonData::with_algorithm(AlgorithmE::parse("unknown"), BpmE::NA);
                let deser = EllingtonData::parse(&"[ed| unknown~na |]".to_string());
                match deser {
                    Some(e) => assert_eq!(ed, e),
                    None => assert!(false),
                }
            }

            #[test]
            fn post() {
                let ed = EllingtonData::with_algorithm(AlgorithmE::parse("unknown"), BpmE::NA);
                let deser = EllingtonData::parse(
                    &"Some, tags, [ed. or other, data [ed| unknown~na |]".to_string(),
                );
                match deser {
                    Some(e) => assert_eq!(ed, e),
                    None => assert!(false),
                }
            }

            #[test]
            fn pre() {
                let ed = EllingtonData::with_algorithm(AlgorithmE::parse("unknown"), BpmE::NA);
                let deser = EllingtonData::parse(
                    &"[ed| unknown~na |] Some, tags, [ed. or other, data".to_string(),
                );
                match deser {
                    Some(e) => assert_eq!(ed, e),
                    None => assert!(false),
                }
            }

            #[test]
            fn mid() {
                let ed = EllingtonData::with_algorithm(AlgorithmE::parse("unknown"), BpmE::NA);
                let deser = EllingtonData::parse(
                    &"Some, tags, [ed. [ed| unknown~na |] or other, data".to_string(),
                );
                match deser {
                    Some(e) => assert_eq!(ed, e),
                    None => assert!(false),
                }
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
    //         let ed = EllingtonData::with_algorithm(AlgorithmE::parse("unknown"), BpmE::Bpm(842));
    //         let comment: String = "".to_string();
    //         let expected: String = "[ed| unknown~842 |]".to_string();

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
