# Ellington [![Build Status](https://travis-ci.org/AdamHarries/ellington.svg?branch=master)](https://travis-ci.org/AdamHarries/ellington)

Ellington is an experimental project to automate the calculation of beats-per-minute (BPM) information for swing jazz music, such as the works of Duke Ellington - the project's namesake. BPM information for swing jazz music is notoriously hard to calculate automatically, as the shuffle of the rhythm section, the soft/loud chunking guitar, and overall swing feel mean that standard algorithms (which are often optimised for four-on-the-floor feel music) report inaccurate times. As such, this project has two main goals: 

1) Provide a platform for experimenting with various BPM algorithms and tools (machine learning anyone?) in order to find high quality (at least >90% accuracy, 99% of the time) solutions. 

2) Provide a tool, or set of tools, for automatically processing libraries of swing jazz music, and reporting BPM information.

## 30-Second overview

Ellington is based around the idea of a 'library' of audio files, that you might wish to process. This is similar to an iTunes library - you must explicitly add audio files to the library, and where possible metadata is written to the library, not the audio file. 

An example ellington usage flow might be: 

    ellington init library.json -d ~/Music/
    ellington bpm library.json 
    ellington write library.json

The above commands, in order: 
  - Initialise a new ellington library `library.json`
  - Calculate bpm information for each audio file in the library, storing the information in `library.json`
  - Write the bpm information to the audio file comment - where requested (more on this later). 

**NOTE: ELLINGTON IS PRE-ALPHA, AND VERY BUGGY. BACK UP YOUR MUSIC LIBRARY BEFORE USING IT TO WRITE METADATA TO AUDIO FILES!!**

## Dependencies

Most *Ellington* dependencies are expressed using the rust package manager, cargo, and so will be automatically installed when *Ellington* is built. However, *Ellington* makes use of a number of external programs for tasks such as parsing mp3 audio data, or writing id3v2 tags. These are: 
  - sox
  - ffmpeg
  - id3v2

With the default build, *Ellington* currently requires *only* ffmpeg. 
  
External programs are listed in `src/shelltools`, in case any are missing here. 

## Detailed Usage 

*Ellington* currently supports three different modes of operations (see `ellington --help` for more information): 

### Library initialisation

Ellington library files can be initialised as follows:

    ellington init <library_file> -d <directory> 
    ellington init <library_file> -i <itunes_xml_library> 
    ellington init <library_file> 

This will write a json-based library to the file given in `<library_file`. Audio discovery is currently possible with three different methodologies. *Ellington* can: 
  - Recursively explore a directory tree to find music files -- `-d <directory>` 
  - Read individual audio file names from stdin -- `-i <itunes_xml_library>`
  - Read an iTunes XML library to discover music files (but not other metadata) -- no further parameters.

### Bpm calculation 

*Ellington* can, given a library file, calculate the bpm of each track in the library using a "pipeline". A pipeline is a combination of an audio decoder (e.g `ffmpeg`), and a bpm algorithm (e.g. `naive`). The results of the bpm calculations are written in place to the library file. This stage can be invoked as follows: 

    ellington bpm <library_file> 

### Metadata writing

*Ellington* files, themselves, are not that useful for the casual DJ. It takes a while to find each song in the JSON, and JSON itself can be a bit tricky to read. In order to remedy this, *Ellington* can write the data that it has calculated to the audio file itself, as follows: 

    ellington write <library_file> 

**NOTE: This will modify metadata of the audio files listed in `library_file`. Run this command at your own risk - it may damage your audio library!**

As the bpms calculated with *Ellington* are not yet high quality, *Ellington* avoids writing to an audio file's `bpm` tag, but instead writes a specially formed piece of text to the comment field of the audio data. *Ellington* is very polite - it only writes to the comment when requested. In fact, all it does is update existing (empty) ellington data. 

Comments with *Ellington* metadata contain a valid *Ellington* data string of the form: 

    [ed#<data>#de]

Where `<data>` is a JSON string, with `:` replaced with `#`, representing some *Ellington* data (see `src/library/ellingtondata.rs`).

A good *default* *Ellington* data string is: 

    [ed#{"algs"#{}}#de]

In order to persuade *Ellington* to write to an audio file, edit the 'comment' metadata tag of it to include the above data string, using your tag editor of choice. 

*Note: id3v2 supports multiple comments for each mp3 file. Ellington, unfortunately, can only see/write one comment per file. If you find that ellington does not write metadata for a particular mp3 file, try deleting all the comments and, using your tag editor of choice, add a single comment containing an Ellington default data string*

## Debugging

By default, *Ellington* is quite conservative in what it prints. In order to get it to log more, export the following environment variable as follows: 

    RUST_LOG=info

## Feature Targets

**0.1.0**: (current master) 
  - Audio file discovery through iTunes based libraries
  - Support for generic audio decoding using `ffmpeg`
  - Support for generic tagging using `taglib`
  - Naive BPM calculation algorithm acting on raw audio data
  - Draft json-based ellington-data format for ephemeral bpm information
  - Audio file discovery through recursive directory enumeration
  - Audio file discovery through `stdin`
  
**0.2.0**: 
  - Stream output/input for libraries (i.e. writing a library to stdout, reading one from stdin - this should allow us to pipe libraries between ellington commands)
  - Comment appending (i.e. programmatically marking tracks as wanting to have bpm information written to them)

**0.2.1**: 
  - Comment "cleaning" for mp3 files (finding mp3 files with multiple comments, and merging/cleaning them.)

**0.3.0**: 
  - Integration of static `ffmpeg` libraries instead of system calls
  - Integration of all dependencies in `cargo.toml`
  - Standalone binary, without dynamic dependencies (including external programs)

**0.4.0**
 - Parallel bpm analysis

**1.0.0**:
  - Stable release of *Ellington*. 
  - Windows support
  
**2.0.0**: 
  - Neural network based bpm classifier
