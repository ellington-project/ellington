# Ellington

Ellington is an experimental project to automate the calculation of beats-per-minute (BPM) information for swing jazz music, such as the works of Duke Ellington - the project's namesake. BPM information for swing jazz music is notoriously hard to calculate automatically, as the two-beat groove and subtle rhythms mean that standard algorithms (which are often optimised for four-on-the-floor feel music) report inaccurate times. As such, this project has two main goals: 

1) Provide a platform for experimenting with various BPM algorithms and tools (machine learning anyone?) in order to find high quality (at least >90% accuracy) solutions. 

2) Provide a tool, or set of tools, for automatically processing libraries of swing jazz music, and reporting BPM information. 

## Current progress. 

The current progress of the project, in terms of the two aims above, is as follows: 

### Experimentation

- Support for the "bpm-tools" implementation of a BPM algorithm. See `tools/bpm-tools/bpm.c` for the implementation. 
- Partial support for audio input using the `sox` audio library.

### Tooling

- Support for iTunes library information, iterating tracks, and passing information to BPM routines. 