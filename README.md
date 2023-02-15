# Web Enum

This is a command line tool for fuzzing URLs using a wordlist. The tool uses Rayon to parallelize requests to the target URL.

## Installation

To use the tool, you will need to have Rust installed on your machine. You can install Rust by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

## Usage

To use the tool, run the following command:

```
cargo run <wordlist_file> <url>
```

where `<wordlist_file>` is the path to a file containing a list of words to use in fuzzing the target URL, and `<url>` is the target URL to fuzz.

By default, the tool will start at the first line of the wordlist. If you want to start at a different line, you can provide the line number as a third argument:

```
cargo run <wordlist_file> <url> <start_line>
```

where `<start_line>` is the line number to start at.

## Dependencies

- `reqwest` for making HTTP requests
- `rayon` for parallelizing requests

