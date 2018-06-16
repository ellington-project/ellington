use std::fmt;

#[derive(Debug)]
pub struct Track {
    pub itunes_id: u32,
    pub bpm: Option<u32>,
    pub name: Option<String>,
    pub location: Option<String>,
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bpm_s = match self.bpm {
            Some(bpm) => format!("{:?}", bpm),
            None => " - ".to_string(),
        };

        let name_s = match self.name {
            Some(name) => name,
            None => " - ".to_string(),
        };

        let location_s = match self.location {
            Some(location) => location,
            None => " - ".to_string(),
        };

        write!(
            f,
            "(id: {} // bpm: {} // name: {} // loc : [...])",
            self.itunes_id, bpm_s, name_s
        )
    }
}
