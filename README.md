# Ellington

Ellington is an experimental project to automate the calculation of beats-per-minute (BPM) information for swing jazz music, such as the works of Duke Ellington - the project's namesake. BPM information for swing jazz music is notoriously hard to calculate automatically, as the two-beat groove and subtle rhythms mean that standard algorithms (which are often optimised for four-on-the-floor feel music) report inaccurate times. As such, this project has two main goals: 

1) Provide a platform for experimenting with various BPM algorithms and tools (machine learning anyone?) in order to find high quality (at least >90% accuracy) solutions. 

2) Provide a tool, or set of tools, for automatically processing libraries of swing jazz music, and reporting BPM information. 

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
  - Standalone binary, without dynamic dependencies (including external programs)

**1.0.0**:
  - Stable release of *Ellington*. 
  - Windows support
  
**2.0.0**: 
  - Neural network based bpm classifier
