use std::fmt;

#[derive(Debug)]
pub struct Track {
    pub itunes_id: i64,
    pub bpm: Option<i64>, // we might not have a bpm value
    pub name: String,
    pub location: String,
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // All this formatting might be slow...
        let bpm_s = match self.bpm {
            Some(bpm) => format!("{:?}", bpm),
            None => " - ".to_string(),
        };
        write!(
            f,
            "(id: {} // bpm: {} // name: {} // loc : [...])",
            self.itunes_id, bpm_s, self.name
        )
    }
}

