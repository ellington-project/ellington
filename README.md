# Ellington [![Build Status](https://travis-ci.org/AdamHarries/ellington.svg?branch=master)](https://travis-ci.org/AdamHarries/ellington)

Ellington is an experimental project to automate the calculation of beats-per-minute (BPM) information for swing jazz music, such as the works of Duke Ellington - the project's namesake. BPM information for swing jazz music is notoriously hard to calculate automatically, as the two-beat groove and subtle rhythms mean that standard algorithms (which are often optimised for four-on-the-floor feel music) report inaccurate times. As such, this project has two main goals: 

1) Provide a platform for experimenting with various BPM algorithms and tools (machine learning anyone?) in order to find high quality (at least >90% accuracy) solutions. 

2) Provide a tool, or set of tools, for automatically processing libraries of swing jazz music, and reporting BPM information.

## Dependencies

Most *Ellington* dependencies are expressed using the rust package manager, cargo, and so will be automatically installed when *Ellington* is built. However, *Ellington* makes use of a number of external programs for tasks such as parsing mp3 audio data, or writing id3v2 tags. These are: 
  - sox
  - ffmpeg
  - id3v2
  
External programs are listed in `src/shelltools`, in case any are missing here. 

## Usage 

*Ellington*, at present, is very limited in its capabilities. It will only currently process mp3 files whose id3v2 comment contains a valid ellington data string, of the form: 

    [ed#<data>#de]

Where `<data>` is a JSON string, with `:` replaced with `#`, representing some ellington data (see `src/library/ellingtondata.rs`).

Audio discovery is currently only possible using a valid iTunes xml library. Ellington can be invoked as follows: 

    cargo run -- --library path/to/iTunes/Library.xml

In future, Ellington will support other media discovery methodologies. 

## Test data

Ellington includes a number of test files in its repository. These files can be found in `$ellington/data`. The files fall (roughly) into one of two categories: 
  - Audio test data, for testing decoding and tagging.
  - Library test data, for testing library enumeration. 

Audio data can be found in `data/flac` and `data/mp3` for flac and mp3 test data, respectively. All audio data is open source, royalty free, and sourced from archive.org. See my [archive.org favourites](https://archive.org/details/fav-harries_adam) for more good royalty free audio files. 

Test data is managed using *git lfs*. This may require extra effort to download 

## Feature Targets

**0.1.0-alpha** (current master): 
  - Audio file discovery through iTunes based libraries
  - mp3 decoding through `SOX` and `ffmpeg`
  - Naive BPM calculation algorithm acting on raw audio data
  - Draft json-based ellington-data format for ephemeral bpm information
  - Tag support for mp3 through id3v2lib. 
  - Support for ellington-data read/write to mp3 files.
  
**0.1.0**: 
  - Audio file discovery through recursive directory enumeration
  - Audio file discovery through `stdin`
  - Support for generic tagging using `taglib`
  - Support for generic audio decoding using `ffmpeg`
  
**0.2.0**: 
  - Integration of static `ffmpeg` libraries instead of system calls
  - Integration of all dependencies in `cargo.toml`
  - Standalone binary, without dynamic dependencies (including external programs)

**1.0.0**:
  - Stable release of *Ellington*. 
  - Windows support
  
**2.0.0**: 
  - Neural network based bpm classifier
