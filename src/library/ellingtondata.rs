use regex::Regex;
use serde_json;
use std::collections::BTreeMap;
use nom;
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct BpmInfo {
//     pub bpm: i64,
//     pub alg: String,
// }

pub type Algorithm = String;
pub type Bpm = i64;

#[derive(Debug)]
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

    pub fn with_algorithm(a: Algorithm, b: Bpm) -> EllingtonData {
        let mut map = BTreeMap::new();
        map.insert(a, b);
        EllingtonData { algs: map }
    }

    pub fn format(self: &Self) -> UpdateResult<String> { 
        let mut s = String::new();
        s.push_str("[ed|");
        let mut first = true;
        for (algorithm, bpm) in self.algs.iter() { 
            if first {
                first = false;
            }else{
                s.push_str(", ");
            }
            s.push_str(&format!("{}~{}", algorithm, bpm));
        }
        s.push_str("|]");
        Ok(s)
    }

    fn regex() -> &'static Regex {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\s*\[ed\|(.*)\|\]").unwrap();
        }
        &RE
    }

    named!(parse_content<&str, Vec<(&str, &str)>>,
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
        match Self::parse_content(captures.get(1)?.as_str()) { 
            Ok((rem, pairs)) => { 
                Some(EllingtonData::empty())
            }
            _ => { 
                println!("Failed to parse ellingotn data from comment!");
                None
            }
        }
    }

    // #[flame]
    pub fn update_data(self: &Self, comment: &String, append: bool) -> UpdateResult<String> {

        let serialised = self.format()?;

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
