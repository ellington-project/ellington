use library::library::*;

pub trait Action<T> {
    fn run(Library) -> Vec<T>;
}

pub struct TrackTitles {}

impl Action<String> for TrackTitles {
    fn run(lib: Library) -> Vec<String> {
        lib.tracks.iter().filter_map(|t| t.name()).collect()
    }
}
