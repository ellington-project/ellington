use std::path::Path;

pub enum AudioFileType {
    Flac,
    M4a,
    M4p,
    Mp3,
    Mp4,
    Wav,
    Alac,
    NotAudio,
}

pub struct FileMetadata {
    pub ftype: AudioFileType,
}

impl FileMetadata {
    pub fn from_path(path: &Path) -> FileMetadata {
        FileMetadata {
            ftype: Self::audio_file_type(path),
        }
    }

    pub fn is_audio_file(path: &Path) -> bool {
        match Self::audio_file_type(path) {
            AudioFileType::NotAudio => false,
            _ => true,
        }
    }

    pub fn seq_audio_file<T>(t: T, path: &Path) -> Option<T> {
        if Self::is_audio_file(path) {
            Some(t)
        } else {
            None
        }
    }

    fn audio_file_type(path: &Path) -> AudioFileType {
        match path.is_dir() {
            true => AudioFileType::NotAudio,
            false => path
                .extension()
                .map_or(AudioFileType::NotAudio, |ext| match ext.to_str() {
                    Some("flac") => AudioFileType::Flac,
                    Some("m4a") => AudioFileType::M4a,
                    Some("m4p") => AudioFileType::M4p,
                    Some("mp3") => AudioFileType::Mp3,
                    Some("mp4") => AudioFileType::Mp4,
                    Some("wav") => AudioFileType::Wav,
                    Some("alac") => AudioFileType::Alac,
                    _ => AudioFileType::NotAudio,
                }),
        }
    }
}
