use library::track::Track;
use regex::Regex;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct BpmInfo {
    pub bpm: f32,
    pub alg: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentData {
    pub algs: Vec<BpmInfo>,
}

impl CommentData {
    pub fn parse_data(track: &Track) -> Option<CommentData> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[ed#(.*)#de\]").unwrap();
        }

        let comment = match &track.comment {
            Some(c) => Some(c),
            None => None,
        };

        println!("Original comment: {}", comment?);

        let captures = RE.captures(comment?.as_str())?;
        // let ellington_data = captures.get(0)?.as_str();
        let json_string = captures.get(1)?.as_str();

        println!("Read: {}", json_string);

        serde_json::from_str(json_string).ok()
    }

    pub fn write_data(self: &Self, track: &Track) -> Option<Track> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[ed#.*#de\]").unwrap();
        }

        let serialised = serde_json::to_string(self).unwrap();

        println!("Serialised: {}", serialised);

        let comment = match &track.comment {
            Some(c) => Some(c.clone()),
            None => None,
        }?;

        println!("Original comment: {}", comment);

        let captures = RE.captures(comment.as_str())?;

        let ellington_data = captures.get(0)?.as_str();
        println!("ellington_data: {}", ellington_data);

        let new_data = format!("[ed#{}#de]", serialised);

        let result = RE.replace(comment.as_str(), new_data.as_str());

        println!("Writing: {}", result);

        Some(Track {
            comment: Some(result.into_owned()),
            ..track.clone()
        })
    }
}
