# imgslice

A basic tool for slicing images into tiles.

## Build

Install a Rust 1.59+ Nightly toolchain.
Clone the git, then in the project root run `cargo build --release`.

## Usage

```txt
imgslice 
Slice an image into tiles, and save them to files

USAGE:
    imgslice.exe [OPTIONS] --input <INPUT> --width <WIDTH> --height <HEIGHT>

OPTIONS:
    -h, --height <HEIGHT>        The height of a tile
        --help                   Print help information
    -i, --input <INPUT>          The path to the input file
    -o, --output <OUTPUT>        The name of the output file
    -w, --width <WIDTH>          The width of a tile
```
