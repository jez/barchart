# barchart

Print a bar chart from the command line.

## Usage

```
❯ cat data.txt
40 label 1
9 label 2
31 something else

❯ barchart data.txt
label 1         40 █████████████████████████████████████████████████████████████
label 2          9 ██████████████
something else  31 ███████████████████████████████████████████████
```

See `barchart --help` for full options.

## Install

From source:

```
cargo install --git https://github.com/jez/barchart.git
```

Precompiled binaries: download from the Releases page, then unzip and put on
your PATH.

## License

[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://jez.io/MIT-LICENSE.txt)

Attribution:

This tool is a Rust port of [jstrace/bars](https://github.com/jstrace/bars),
which is also MIT licensed.
