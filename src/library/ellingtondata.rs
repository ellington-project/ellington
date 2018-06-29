use regex::Regex;
use serde_json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BpmInfo {
    pub bpm: f32,
    pub alg: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EllingtonData {
    pub algs: Vec<BpmInfo>,
}

impl EllingtonData {
    #[flame]
    pub fn parse_data(comment: &String) -> Option<EllingtonData> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[ed#(.*)#de\]").unwrap();
        }

        println!("Original comment: {}", comment);

        let captures = RE.captures(comment.as_str())?;
        
        let json_string = captures.get(1)?.as_str();

        println!("Read data: {}", json_string);

        serde_json::from_str(json_string).ok()
    }

    #[flame]
    pub fn update_data(self: &Self, comment: &String) -> Option<String> { 
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[ed#.*#de\]").unwrap();
        }

        let serialised = serde_json::to_string(self).unwrap();

        println!("Serialised: {}", serialised);

        println!("Original comment: {}", comment);

        let captures = RE.captures(comment.as_str())?;

        let ellington_data = captures.get(0)?.as_str();

        println!("ellington_data: {}", ellington_data);

        let new_data = format!("[ed#{}#de]", serialised);

        let result = RE.replace(comment.as_str(), new_data.as_str());

        println!("Writing: {}", result);

        Some(result.to_string())
    }
}
