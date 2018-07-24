use regex::Regex;
use serde_json;
use std::collections::{BTreeMap};

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct BpmInfo {
//     pub bpm: i64,
//     pub alg: String,
// }

pub type Algorithm = String;
pub type Bpm = i64;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EllingtonData {
    pub algs: BTreeMap<Algorithm, Bpm>
    // pub algs: Vec<BpmInfo>,
}

impl EllingtonData {
    pub fn empty() -> EllingtonData {
        EllingtonData { algs: BTreeMap::new() }
    }
    // #[flame]
    pub fn parse_data(comment: &String) -> Option<EllingtonData> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[ed#(.*)#de\]").unwrap();
        }

        let captures = RE.captures(comment.as_str())?;

        let json_string = captures.get(1)?.as_str().replace("#", ":");

        serde_json::from_str(&json_string).ok()
    }

    // #[flame]
    pub fn update_data(self: &Self, comment: &String) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[ed#.*#de\]").unwrap();
        }

        // replace all the ":" characters in the JSON string with "#", as id3tags do not support colons in comment data.
        let serialised = serde_json::to_string(self).unwrap().replace(":", "#");

        // let captures = RE.captures(comment.as_str())?;

        // let ellington_data = captures.get(0)?.as_str();

        let new_data = format!("[ed#{}#de]", serialised);

        let result = RE.replace(comment.as_str(), new_data.as_str());

        Some(result.to_string())
    }
}
