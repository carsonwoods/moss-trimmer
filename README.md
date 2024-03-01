# MOSS Trimmer
This is a very simple utility built around making [MOSS](https://theory.stanford.edu/~aiken/moss/), simpler to use.
This tool can be used to download MOSS results so that they are stored entirely offline.
This can be useful if results are needed to persist for longer than the 2 week storage time provided by MOSS servers.

Additionally, by passing a `-t <STRING>`, argument, the tool will remove all results whose name does not include the string.
This can be used to ignore similarity results identified between two files in a code archive.


### Requirements
- Rust
- wget

### Installation Instructions
Moss Trimmer can be installed using Cargo:
`cargo install moss-trimmer`
