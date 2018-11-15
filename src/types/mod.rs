use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlgorithmE {
    Actual,
    Naive,
    Bellson,
    Unknown,
}

impl AlgorithmE {
    pub fn parse(st: &str) -> AlgorithmE {
        match st.to_lowercase().chars().next().unwrap() {
            'a' => AlgorithmE::Actual,
            'n' => AlgorithmE::Naive,
            'b' => AlgorithmE::Bellson,
            _ => AlgorithmE::Unknown,
        }
    }

    pub fn print(&self) -> &'static str {
        match self {
            AlgorithmE::Actual => "actual",
            AlgorithmE::Naive => "naive",
            AlgorithmE::Bellson => "bellson",
            AlgorithmE::Unknown => "unknown",
        }
    }
}

impl fmt::Display for AlgorithmE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AlgorithmE::Actual => write!(f, "actual"),
            AlgorithmE::Naive => write!(f, "naive"),
            AlgorithmE::Bellson => write!(f, "bellson"),
            AlgorithmE::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BpmE {
    Bpm(i64),
    NA,
}

impl BpmE {
    pub fn parse(st: &str) -> BpmE {
        if st == "na" {
            BpmE::NA
        } else {
            match st.parse::<i64>() {
                Ok(tmpo) => BpmE::Bpm(tmpo),
                Err(e) => {
                    error!("Got error {:?} while parsing integer from {}", e, st);
                    BpmE::NA
                }
            }
        }
    }

    pub fn from_option(i: Option<i64>) -> BpmE {
        match i {
            Some(bpm) => BpmE::Bpm(bpm),
            None => BpmE::NA,
        }
    }
}

impl fmt::Display for BpmE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BpmE::Bpm(tmpo) => write!(f, "{}", tmpo),
            BpmE::NA => write!(f, "na"),
        }
    }
}

pub enum UpdateBehaviour {
    FailIfNone,
    Append,
    Prepend,
}

impl UpdateBehaviour {
    pub fn parse(st: &str) -> UpdateBehaviour {
        match st {
            "append" => UpdateBehaviour::Append,
            "prepend" => UpdateBehaviour::Prepend,
            _ => UpdateBehaviour::FailIfNone,
        }
    }
}
