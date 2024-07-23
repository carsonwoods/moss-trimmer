# MOSS Trimmer
This is a very simple utility built around making [MOSS](https://theory.stanford.edu/~aiken/moss/), simpler to use.
This tool can be used to download MOSS results so that they are stored entirely offline.
This can be useful if results are needed to persist for longer than the 2-week storage time provided by MOSS servers.

Additionally, by passing a `-t <STRING>`, argument, the tool will remove all results whose name does not include the string.
This can be used to ignore similarity results identified between two files in a code archive.

**NOTICE:** This tool was designed and intended to aid professors and TAs who rely on MOSS for plagiarism checking in an academic setting. Anyone is free to use it, but please **DO NOT** use it to try and scrape results that were not submitted by you in-order to reduce MOSS server load.


### Requirements
- Rust
- wget

### Installation Instructions
Moss Trimmer can be installed using Cargo:
`cargo install moss-trimmer`
