use regex::Regex;
use serde_json;
use std::collections::BTreeMap;

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct BpmInfo {
//     pub bpm: i64,
//     pub alg: String,
// }

pub type Algorithm = String;
pub type Bpm = i64;

pub enum UpdateError {
    NoDataInComment,
    FailedToSerialise,
}
pub type UpdateResult<T> = Result<T, UpdateError>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EllingtonData {
    pub algs: BTreeMap<Algorithm, Bpm>, // pub algs: Vec<BpmInfo>
}

impl EllingtonData {
    pub fn empty() -> EllingtonData {
        EllingtonData {
            algs: BTreeMap::new(),
        }
    }

    fn serialise(self: &Self) -> UpdateResult<String> {
        serde_json::to_string(self)
            .ok()
            .and_then(|s| Some(s.replace(":", "#")))
            .ok_or(UpdateError::FailedToSerialise)
            .and_then(|s| Ok(format!("[ed#{}#de]", s)))
    }

    fn regex() -> &'static Regex {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\s*\[ed#(.*)#de\]").unwrap();
        }
        &RE
    }

    // #[flame]
    pub fn parse_data(comment: &String) -> Option<EllingtonData> {
        let captures = Self::regex().captures(comment.as_str())?;

        let json_string = captures.get(1)?.as_str().replace("#", ":");

        serde_json::from_str(&json_string).ok()
    }

    // #[flame]
    pub fn update_data(self: &Self, comment: &String, append: bool) -> UpdateResult<String> {
        // replace all the ":" characters in the JSON string with "#", as id3tags do not support colons in comment data.
        let serialised = self.serialise()?;

        // test to see if there is any json data in the first place...
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
